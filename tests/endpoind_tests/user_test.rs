use std::io::Read;

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use models::{
    permissions::Permission,
    users::{PasswordCredentials, UserExternal},
};
use persistence::PostgresDb;

use service_starter_rs::app;
use tower::ServiceExt;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

#[tokio::test]
async fn singup_test() {
    let user_to_create = PasswordCredentials {
        username: "testUser".to_string(),
        password: "testPass".to_string(),
        next: None,
    };
    let default_permissions = Permission::new("user".to_string());
    let created_user = UserExternal::new(
        Uuid::new_v4(),
        user_to_create.username.clone(),
        default_permissions,
    );
    let db = PostgresDb::new().await;
    let session_store = PostgresStore::new(db.conn_pool.clone());
    let app = app(db, session_store);
    let request = Request::builder()
        .uri("/api/users/signup")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(format!(
            "username={}&password={}",
            user_to_create.username, user_to_create.password
        )))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().contains_key("Set-Cookie"));
    let res_body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let res_body_slice: Vec<u8> = res_body_bytes.bytes().map(|b| b.unwrap()).collect();
    let res_body_json: UserExternal = serde_json::from_slice(res_body_slice.as_slice()).unwrap();
    assert_eq!(created_user.username, res_body_json.username);
}
