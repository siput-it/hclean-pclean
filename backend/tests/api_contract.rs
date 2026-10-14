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
async fn api_version_returns_standard_success_envelope() {
    let response = send(Method::GET, "/api/v1", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["version"], "v1");
    assert_eq!(json["data"]["message"], "API ready");
}

#[tokio::test]
async fn service_catalog_contains_all_simulated_categories_with_integer_prices() {
    let response = send(Method::GET, "/api/v1/services", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["simulated"], true);

    let items = json["data"]["items"].as_array().unwrap();
    assert_eq!(items.len(), 7);
    assert_eq!(
        items
            .iter()
            .map(|item| item["name"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "Membersihkan toilet",
            "Membersihkan kasur",
            "Mencuci pakaian",
            "Menyetrika",
            "Cuci dan setrika",
            "Berkebun",
            "Maintenance/perbaikan rumah",
        ]
    );
    assert!(items
        .iter()
        .all(|item| { item["simulated"] == true && item["price_rupiah"].as_u64().is_some() }));

    let detail = send(Method::GET, "/api/v1/services/laundry", None).await;
    assert_eq!(detail.status(), StatusCode::OK);
    assert_eq!(response_json(detail).await["data"]["id"], "laundry");
}

#[tokio::test]
async fn invalid_registration_has_standard_validation_error() {
    let response = send(
        Method::POST,
        "/api/v1/auth/register",
        Some(r#"{"name":"","email":"invalid","password":"short","role":"customer"}"#),
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "VALIDATION_ERROR");
    assert_eq!(json["error"]["message"], "Data permintaan tidak valid");
}

#[tokio::test]
async fn valid_registration_is_not_claimed_as_persisted() {
    let response = send(
        Method::POST,
        "/api/v1/auth/register",
        Some(r#"{"name":"Pelanggan","email":"user@example.test","password":"password-aman","role":"customer"}"#),
    )
    .await;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "DATABASE_UNAVAILABLE");
    assert!(!json.to_string().contains("password-aman"));
}

#[tokio::test]
async fn wallet_is_explicitly_simulated() {
    let response = send(Method::GET, "/api/v1/wallet", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["data"]["simulated"], true);
    assert_eq!(json["data"]["is_real_money"], false);
    assert_eq!(json["data"]["balance_rupiah"].as_u64(), Some(0));

    let response = send(Method::GET, "/api/v1/wallet/transactions", None).await;
    let json = response_json(response).await;
    assert_eq!(json["data"]["simulated"], true);
    assert_eq!(json["data"]["transactions"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn order_input_is_validated_but_valid_order_requires_authentication() {
    let invalid = send(
        Method::POST,
        "/api/v1/orders",
        Some(r#"{"service_id":"unknown","scheduled_at":"","address":""}"#),
    )
    .await;
    assert_eq!(invalid.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(invalid).await["error"]["code"],
        "UNAUTHORIZED"
    );

    let valid = send(
        Method::POST,
        "/api/v1/orders",
        Some(r#"{"service_id":"laundry","scheduled_at":"2026-10-20T10:00:00+07:00","address":"Jl. Contoh 1","notes":""}"#),
    )
    .await;
    assert_eq!(valid.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response_json(valid).await["error"]["code"], "UNAUTHORIZED");
}

#[tokio::test]
async fn unsupported_identity_and_unknown_routes_have_explicit_statuses() {
    let profile = send(Method::GET, "/api/v1/users/me", None).await;
    assert_eq!(profile.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(profile).await["error"]["code"],
        "UNAUTHORIZED"
    );

    let missing_service = send(Method::GET, "/api/v1/services/missing", None).await;
    assert_eq!(missing_service.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        response_json(missing_service).await["error"]["code"],
        "SERVICE_NOT_FOUND"
    );

    let missing_route = send(Method::GET, "/api/v1/does-not-exist", None).await;
    assert_eq!(missing_route.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        response_json(missing_route).await["error"]["code"],
        "NOT_FOUND"
    );

    let wrong_method = send(Method::POST, "/health", None).await;
    assert_eq!(wrong_method.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(
        response_json(wrong_method).await["error"]["code"],
        "METHOD_NOT_ALLOWED"
    );
}

#[tokio::test]
async fn order_routes_require_authentication_before_business_processing() {
    let routes = [
        (Method::GET, "/api/v1/orders", None),
        (Method::GET, "/api/v1/orders/order-1", None),
        (Method::POST, "/api/v1/orders/order-1/accept", None),
        (Method::POST, "/api/v1/orders/order-1/start", None),
        (Method::POST, "/api/v1/orders/order-1/complete", None),
        (Method::POST, "/api/v1/orders/order-1/cancel", None),
    ];

    for (method, uri, body) in routes {
        let response = send(method, uri, body).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{uri}");

        let json = response_json(response).await;
        assert_eq!(json["success"], false, "{uri}");
        assert_eq!(json["error"]["code"], "UNAUTHORIZED", "{uri}");
    }
}
