use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use reqwest::StatusCode;

pub async fn admin_auth(
    req: Request<Body>,
    next: Next,
    admin_token: String,
) -> axum::response::Response {
    let request_token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    if request_token != Some(&admin_token) {
        eprintln!("Unauthorized request to admin endpoint");
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap();
        return response;
    }

    next.run(req).await
}

