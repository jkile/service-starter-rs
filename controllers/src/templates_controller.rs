use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::AppState;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "pages/login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "pages/signup.html")]
struct SignupTemplate;

#[derive(Template)]
#[template(path = "pages/404.html")]
struct FallbackTemplate;

struct HtmlTemplate<T>(T);

pub fn collect_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_home))
        .route("/login", get(login))
        .route("/signup", get(signup))
}

async fn get_home(State(_app_state): State<AppState>) -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

async fn login(State(_app_state): State<AppState>) -> impl IntoResponse {
    let template = LoginTemplate {};
    HtmlTemplate(template)
}

async fn signup(State(_app_state): State<AppState>) -> impl IntoResponse {
    let template = SignupTemplate {};
    HtmlTemplate(template)
}

pub async fn handle_404(State(_app_state): State<AppState>) -> impl IntoResponse {
    let template = FallbackTemplate {};
    HtmlTemplate(template)
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
