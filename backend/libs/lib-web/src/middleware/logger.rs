use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;

/// Request logging middleware
pub async fn logger_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info!("→ {} {}", method, uri);

    let response = next.run(req).await;

    info!("← {} {} - {}", method, uri, response.status());

    response
}
