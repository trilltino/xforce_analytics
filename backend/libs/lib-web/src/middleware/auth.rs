use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use tower_cookies::Cookies;
use uuid::Uuid;

use lib_core::ctx::UserCtx;

pub const AUTH_TOKEN_COOKIE: &str = "auth_token";

/// Middleware to extract and validate user authentication
pub async fn auth_middleware(
    State(pool): State<PgPool>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get auth token from cookie
    let token = cookies
        .get(AUTH_TOKEN_COOKIE)
        .map(|c| c.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate token and get user
    let user_ctx = validate_token_and_get_user(&pool, &token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Insert user context into request extensions
    req.extensions_mut().insert(user_ctx);

    Ok(next.run(req).await)
}

/// Validate token and retrieve user context
async fn validate_token_and_get_user(
    pool: &PgPool,
    token: &str,
) -> Result<UserCtx, String> {
    use lib_auth::hash_token;

    let token_hash = hash_token(token);

    // Query to get user from valid session
    let result = sqlx::query_as::<_, (Uuid, String)>(
        r#"
        SELECT u.id, u.email
        FROM users u
        INNER JOIN sessions s ON s.user_id = u.id
        WHERE s.token_hash = $1
        AND s.expires_at > NOW()
        AND u.is_active = true
        "#
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match result {
        Some((user_id, email)) => Ok(UserCtx::new(user_id, email)),
        None => Err("Invalid or expired session".to_string()),
    }
}
