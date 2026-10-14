use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use hclean_pclean_backend::http::routes::router;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_backend_status_without_database_claim() {
    let response = router()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should handle health request");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body = to_bytes(response.into_body(), 1024)
        .await
        .expect("response body should be readable");
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("response body should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["status"], "ok");
    assert_eq!(json["data"]["service"], "hclean-pclean-backend");
    assert!(json["data"].get("database").is_none());
}
