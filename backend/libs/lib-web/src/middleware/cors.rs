use tower_http::cors::{AllowOrigin, CorsLayer};
use axum::http::{HeaderValue, Method};

/// Create CORS layer for development
pub fn cors_dev() -> CorsLayer {
    CorsLayer::permissive()
}

/// Create CORS layer for production
pub fn cors_production(allowed_origins: Vec<String>) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .into_iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
}
