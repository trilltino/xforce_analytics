use sqlx::PgPool;
use lib_auth::{hash_password, hash_token, verify_password, generate_session_token, HashScheme};
use lib_core::AuthConfig;
use lib_utils::{new_uuid, now_plus_days, validate_email, validate_password};
use lib_web::AppError;
use shared::{LoginRequest, SignupRequest, User};
use uuid::Uuid;

use crate::repositories::UserRepository;

pub struct AuthService;

impl AuthService {
    /// Sign up a new user
    pub async fn signup(
        pool: &PgPool,
        req: SignupRequest,
    ) -> Result<(User, String), AppError> {
        // Validate input
        validate_email(&req.email)
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        let auth_config = AuthConfig::from_env();
        validate_password(&req.password, auth_config.password_min_length)
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        // Check if user already exists
        if UserRepository::find_by_email(pool, &req.email).await.is_ok() {
            return Err(AppError::BadRequest(
                "User with this email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = hash_password(&req.password, HashScheme::Argon2)
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        // Create user
        let user_id = new_uuid();
        let user = UserRepository::create(
            pool,
            user_id,
            &req.email,
            &password_hash,
            req.full_name.as_deref(),
        )
        .await?;

        // Create session
        let token = generate_session_token();
        let token_hash = hash_token(&token);
        let expires_at = now_plus_days(auth_config.token_duration_days);

        Self::create_session(pool, user_id, &token_hash, expires_at).await?;

        Ok((user, token))
    }

    /// Log in a user
    pub async fn login(
        pool: &PgPool,
        req: LoginRequest,
    ) -> Result<(User, String), AppError> {
        // Find user by email
        let user_for_auth = UserRepository::find_by_email(pool, &req.email)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // Verify password
        let is_valid = verify_password(&req.password, &user_for_auth.password_hash)
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Get full user info
        let user = UserRepository::find_by_id(pool, user_for_auth.id).await?;

        // Update last login
        UserRepository::update_last_login(pool, user.id).await?;

        // Create new session
        let auth_config = AuthConfig::from_env();
        let token = generate_session_token();
        let token_hash = hash_token(&token);
        let expires_at = now_plus_days(auth_config.token_duration_days);

        Self::create_session(pool, user.id, &token_hash, expires_at).await?;

        Ok((user, token))
    }

    /// Log out a user (delete session)
    pub async fn logout(pool: &PgPool, token: &str) -> Result<(), AppError> {
        let token_hash = hash_token(token);

        sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE token_hash = $1
            "#
        )
        .bind(&token_hash)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Create a session
    async fn create_session(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError> {
        let session_id = new_uuid();

        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, token_hash, expires_at)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(session_id)
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(pool)
        .await?;

        Ok(())
    }
}
