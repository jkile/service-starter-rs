use std::borrow::BorrowMut;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use persistence::postgres_db::PostgresDb;

use service_starter_rs::app;
use sqlx::PgPool;
use tower::ServiceExt;
use tower_sessions_sqlx_store::PostgresStore;

use crate::common;

#[sqlx::test(migrations = "./persistence/migrations")]
async fn singup_test(pool: PgPool) {
    let db = PostgresDb::from_pool(pool).await;
    let session_store = PostgresStore::new(db.conn_pool.clone());
    if let Err(err) = session_store.migrate().await {
        panic!("{}", err)
    }

    let mut app = app(db, session_store);

    // Happy path
    let request_one = Request::builder()
        .uri("/api/users/signup")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(format!("username=testUser&password=testPass")))
        .unwrap();

    let response_one = app.borrow_mut().oneshot(request_one).await.unwrap();

    assert_eq!(response_one.status(), StatusCode::OK);
    assert!(response_one.headers().contains_key("Set-Cookie"));
    let res_body_json = common::response_json(response_one).await;
    assert_eq!("testUser", res_body_json["username"]);

    // Invalid username
    let request_two = Request::builder()
        .uri("/api/users/signup")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(format!("username=b&password=testPass")))
        .unwrap();

    let response_two = app.borrow_mut().oneshot(request_two).await.unwrap();
    assert_eq!(response_two.status(), StatusCode::BAD_REQUEST);

    // Invalid password
    let request_three = Request::builder()
        .uri("/api/users/signup")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(format!("username=testUserTwo&password=t")))
        .unwrap();

    let response_three = app.borrow_mut().oneshot(request_three).await.unwrap();
    assert_eq!(response_three.status(), StatusCode::BAD_REQUEST);

    // Username is not unique
    let request_four = Request::builder()
        .uri("/api/users/signup")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(format!("username=testUser&password=testPass")))
        .unwrap();

    let response_four = app.borrow_mut().oneshot(request_four).await.unwrap();
    assert_eq!(response_four.status(), StatusCode::CONFLICT);
}
