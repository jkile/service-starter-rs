use axum::{
    body::Body,
    http::{header, Request, Response, StatusCode},
};
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use tower_http::auth::AsyncAuthorizeRequest;

#[derive(Clone, Copy)]
pub struct ClerkAuth;

impl<B> AsyncAuthorizeRequest<B> for ClerkAuth
where
    B: Send + 'static,
{
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;
    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async move {
            let mut keys: Vec<ClerkJWKey> = Vec::new();

            let clerk_secret = std::env::var("CLERK_SECRET_KEY").unwrap();
            let client = reqwest::Client::new();
            let clerk_request = client
                .get("https://api.clerk.dev/v1/jwks")
                .bearer_auth(clerk_secret)
                .send()
                .await;
            if let Ok(response) = clerk_request {
                let response = response.json::<ClerkJWKSResponse>().await.unwrap();
                response.keys.iter().for_each(|key| keys.push(key.clone()));
            };

            let auth_token = request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|it| it.to_str().ok())
                .and_then(|it| it.strip_prefix("Bearer "));
            if let None = auth_token {
                return Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap());
            }

            let temp_key = keys.first().unwrap();
            if let Ok(decoding_key) = jsonwebtoken::DecodingKey::from_rsa_components(
                temp_key.n.as_str(),
                temp_key.e.as_str(),
            ) {
                let token_contents = decode::<ClerkClaims>(
                    auth_token.unwrap(),
                    &decoding_key,
                    &Validation::new(jsonwebtoken::Algorithm::RS256),
                );

                if let Ok(_token_contents) = token_contents {
                    // TODO: Validate issuer and azp to make sure origin is correct
                    // also validate exp (expiration time) hasn't passed and nbf (not before) has in fact passed :)
                    Ok(request)
                } else {
                    Err(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap())
                }
            } else {
                Err(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap())
            }
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClerkClaims {
    azp: String,
    exp: i32,
    iat: i32,
    iss: String,
    nbf: i32,
    sub: String,
}

#[derive(Debug, Deserialize)]
pub struct ClerkJWKSResponse {
    keys: Vec<ClerkJWKey>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClerkJWKey {
    #[serde(rename = "use")]
    pub use_name: String,
    pub kty: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}
