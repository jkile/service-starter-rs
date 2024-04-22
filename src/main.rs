use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderMap, HeaderName, HeaderValue, Method, Request};
use axum::{response::Response, Router};
use bytes::Bytes;
use controllers::{self};
use notify::Watcher;
use std::path::Path;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing::Level;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "service=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing router...");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let db = persistence::Db::new().await;
    let app_state = controllers::AppState { db };

    let assets_path = std::env::current_dir().unwrap();

    let live_reload = LiveReloadLayer::new();
    let reloader = live_reload.reloader();

    let router = Router::new()
        .merge(controllers::collect_template_routes())
        .nest_service(
            "/assets",
            ServeDir::new(format!(
                "{}/controllers/templates/assets",
                assets_path.to_str().unwrap()
            )),
        )
        .nest_service(
            "/scripts",
            ServeDir::new(format!(
                "{}/controllers/templates/scripts",
                assets_path.to_str().unwrap()
            )),
        )
        .nest("/api", controllers::collect_api_routes())
        .merge(controllers::collect_fallback_route())
        .with_state(app_state)
        .layer(live_reload.request_predicate(not_htmx_predicate))
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

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(
        Path::new("./controllers/templates"),
        notify::RecursiveMode::Recursive,
    )?;

    let port = 8080_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("Router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

fn not_htmx_predicate<T>(req: &Request<T>) -> bool {
    !req.headers().contains_key("hx-request")
}
