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
async fn authenticated_access_is_required_for_addresses_and_worker_profile() {
    let addresses = send(Method::GET, "/api/v1/addresses", None).await;
    assert_eq!(addresses.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(addresses).await["error"]["code"],
        "UNAUTHORIZED"
    );

    let worker_profile = send(Method::GET, "/api/v1/workers/me", None).await;
    assert_eq!(worker_profile.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(worker_profile).await["error"]["code"],
        "UNAUTHORIZED"
    );
}
