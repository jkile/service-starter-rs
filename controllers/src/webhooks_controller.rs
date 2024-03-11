use axum::{http::HeaderMap, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use utils::error::ApplicationError;

use crate::AppState;

pub fn collect_routes() -> Router<AppState> {
    Router::new().route("/clerk", post(clerk_webhook))
}

fn verify_webhook(headers: &HeaderMap, body: &[u8]) -> Result<(), ApplicationError> {
    let secret_key = std::env::var("SVIX_SECRET_KEY").unwrap();
    let wh = svix::webhooks::Webhook::new(secret_key.as_str());
    match wh {
        Ok(wh) => match wh.verify(body, headers) {
            Ok(_) => Ok(()),
            Err(e) => Err(ApplicationError::FailureVerifyingWebookSignature(e)),
        },
        Err(e) => Err(ApplicationError::FailureVerifyingWebookSignature(e)),
    }
}

async fn clerk_webhook(
    headers: HeaderMap,
    // State(app_state): State<AppState>,
    Json(payload): Json<ClerkCreateUserPayload>,
) -> Result<Json<ClerkCreateUserPayload>, ApplicationError> {
    // Parse json body into bytes to verify signature of webhook as required by Svix
    // There is probably a less ugly way of doing this but for now, it works
    let mut bytes: Vec<u8> = Vec::new();
    serde_json::to_writer::<&mut Vec<u8>, _>(&mut bytes, &payload).unwrap();
    if let Err(e) = verify_webhook(&headers, bytes.as_slice()) {
        return Err(e);
    }

    //user_service::create_user(app_state.db, user);
    Ok(Json(payload))
}

#[derive(Debug, Serialize, Deserialize)]
struct ClerkCreateUserPayload {
    data: ClerkCreateUserPayloadData,
    object: String,
    #[serde(rename = "type")]
    type_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ClerkCreateUserPayloadData {
    birthday: Option<String>,
    created_at: u64,
    email_addresses: Vec<ClerkEmailAddress>,
    external_accounts: Vec<String>,
    external_id: Option<String>,
    first_name: Option<String>,
    gender: Option<String>,
    id: String,
    image_url: Option<String>,
    last_name: Option<String>,
    last_sign_in_at: Option<u64>,
    object: Option<String>,
    password_enabled: bool,
    phone_numbers: Vec<String>,
    primary_email_address_id: Option<String>,
    primary_phone_number_id: Option<String>,
    primary_web3_wallet_id: Option<String>,
    private_metadata: Option<ClerkPrivateMetaData>,
    profile_image_url: Option<String>,
    public_metadata: Option<ClerkPublicMetaData>,
    two_factor_enabled: bool,
    unsafe_metadata: Option<ClerkUnsafecMetaData>,
    updated_at: Option<u64>,
    username: Option<String>,
    web3_wallets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClerkEmailAddress {
    email_address: String,
    id: String,
    linked_to: Vec<String>,
    object: String,
    verification: Option<ClerkEmailVerification>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClerkEmailVerification {
    status: String,
    strategy: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClerkPrivateMetaData {}

#[derive(Debug, Serialize, Deserialize)]
struct ClerkPublicMetaData {}

#[derive(Debug, Serialize, Deserialize)]
struct ClerkUnsafecMetaData {}
