use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderName, HeaderValue, Method, Request};
use axum::{http::HeaderMap, response::Response, Router};
use axum_login::tower_sessions::SessionManagerLayer;
use axum_login::AuthManagerLayerBuilder;
use bytes::Bytes;
use controllers::{self};
use dotenvy::dotenv;
use std::time::Duration;
use time;
use tokio::signal;
use tokio::task::AbortHandle;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_sessions_core::{ExpiredDeletion, Expiry};
use tracing::info;
use tracing::Level;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().unwrap();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "service=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    info!("Initializing router...");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let db = persistence::PostgresDb::new().await;
    let deletion_task = tokio::task::spawn(
        db.session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(db.session_store.clone())
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(3600)));
    let auth_layer = AuthManagerLayerBuilder::new(db.clone(), session_layer).build();
    let app_state = controllers::AppState { db };

    let router = Router::new()
        .nest("/api", controllers::collect_routes())
        .with_state(app_state)
        .layer(auth_layer)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = Uuid::new_v4();
                    tracing::span!(
                        Level::INFO,
                        "http-request",
                        method = tracing::field::display(request.method()),
                        uri = tracing::field::display(request.uri().path()),
                        version = tracing::field::debug(request.version()),
                        headers = tracing::field::debug(
                            request
                                .headers()
                                .iter()
                                .filter(|header| header.0 != COOKIE)
                                .collect::<Vec<(&HeaderName, &HeaderValue)>>()
                        ),
                        request_id = tracing::field::display(request_id)
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::info!("started {} {}", request.method(), request.uri().path())
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    tracing::info!(
                        "response generated in {:?} with response code: {:?}",
                        latency,
                        response.status()
                    )
                })
                .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                    tracing::debug!("sending {} bytes", chunk.len())
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("something went wrong")
                    },
                ),
        )
        .layer(CompressionLayer::new());

    let port = 8080_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("Router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
