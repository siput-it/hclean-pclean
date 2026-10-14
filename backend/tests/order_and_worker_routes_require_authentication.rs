use axum::{
    body::{to_bytes, Body},
    http::{Method, Request, Response, StatusCode},
};
use hclean_pclean_backend::http::routes::router;
use serde_json::Value;
use tower::ServiceExt;

async fn send(method: Method, uri: &str, body: Option<&str>) -> Response<Body> {
    let mut request = Request::builder().method(method).uri(uri);
    if body.is_some() {
        request = request.header("content-type", "application/json");
    }

    router()
        .oneshot(
            request
                .body(Body::from(body.unwrap_or_default().to_owned()))
                .expect("request should be valid"),
        )
        .await
        .expect("router should handle request")
}

async fn response_json(response: Response<Body>) -> Value {
    let body = to_bytes(response.into_body(), 16 * 1024)
        .await
        .expect("response body should be readable");
    serde_json::from_slice(&body).expect("response body should be valid JSON")
}

#[tokio::test]
async fn order_and_worker_routes_require_authentication() {
    let create = send(
        Method::POST,
        "/api/v1/orders",
        Some(r#"{"service_category_id":"11111111-1111-1111-1111-111111111111","address_id":"11111111-1111-1111-1111-111111111111","notes":"test"}"#),
    )
    .await;
    assert_eq!(create.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response_json(create).await["error"]["code"], "UNAUTHORIZED");

    let available = send(Method::GET, "/api/v1/worker/orders/available", None).await;
    assert_eq!(available.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(available).await["error"]["code"],
        "UNAUTHORIZED"
    );
}
