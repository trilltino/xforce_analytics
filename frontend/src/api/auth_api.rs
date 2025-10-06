use shared::{AuthResponse, LoginRequest, LogoutResponse, SignupRequest};
use super::client;

pub async fn signup(request: SignupRequest) -> Result<AuthResponse, String> {
    client::post("/api/auth/signup", &request).await
}

pub async fn login(request: LoginRequest) -> Result<AuthResponse, String> {
    client::post("/api/auth/login", &request).await
}

pub async fn logout() -> Result<LogoutResponse, String> {
    client::post("/api/auth/logout", &()).await
}
