use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderName, HeaderValue, Method, Request};
use axum::{http::HeaderMap, response::Response, Router};
use axum_login::tower_sessions::SessionManagerLayer;
use axum_login::AuthManagerLayerBuilder;
use bytes::Bytes;
use controllers::{self, AppState};
use persistence::Db;
use std::time::Duration;
use time;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_sessions_core::Expiry;
use tracing::Level;
use tracing::Span;
use uuid::Uuid;

pub fn app<T: Db + 'static>(app_state: AppState<T>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
    let session_layer = SessionManagerLayer::new(app_state.db.get_session_store())
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(3600)));
    let auth_layer = AuthManagerLayerBuilder::new(app_state.db.clone(), session_layer).build();

    Router::new()
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
        .layer(CompressionLayer::new())
}