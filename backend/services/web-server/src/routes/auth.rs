use axum::{extract::State, response::IntoResponse, Json};
use lib_web::{success, AppError};
use shared::{AuthResponse, LoginRequest, LogoutResponse, SignupRequest};
use tower_cookies::Cookies;

use crate::{services::AuthService, AppState};

pub async fn signup(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(req): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (user, token) = AuthService::signup(&state.db, req).await?;

    // Set auth cookie
    let cookie = tower_cookies::Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .build();

    cookies.add(cookie);

    Ok(success(
        AuthResponse {
            user,
            message: "Signup successful".to_string(),
        },
        "Account created successfully",
    ))
}

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (user, token) = AuthService::login(&state.db, req).await?;

    // Set auth cookie
    let cookie = tower_cookies::Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .build();

    cookies.add(cookie);

    Ok(success(
        AuthResponse {
            user,
            message: "Login successful".to_string(),
        },
        "Logged in successfully",
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    // Get token from cookie
    if let Some(cookie) = cookies.get("auth_token") {
        let token = cookie.value();
        // Delete session from database
        AuthService::logout(&state.db, token).await?;
    }

    // Remove cookie
    cookies.remove(tower_cookies::Cookie::from("auth_token"));

    Ok(success(
        LogoutResponse {
            message: "Logout successful".to_string(),
        },
        "Logged out successfully",
    ))
}
