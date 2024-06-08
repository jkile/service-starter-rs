use axum::{
    body::{to_bytes, Body},
    http::{header::CONTENT_TYPE, Response},
};
use std::io::Read;

pub async fn response_json(response: Response<Body>) -> serde_json::Value {
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .expect("expected Content-Type"),
        "application/json"
    );

    let res_body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let res_body_slice: Vec<u8> = res_body_bytes.bytes().map(|b| b.unwrap()).collect();
    let res_body_json = serde_json::from_slice(res_body_slice.as_slice()).unwrap();
    res_body_json
}
