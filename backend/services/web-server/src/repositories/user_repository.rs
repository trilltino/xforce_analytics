use sqlx::PgPool;
use uuid::Uuid;
use lib_utils::now;
use lib_web::AppError;
use shared::{User, UserForAuth};

pub struct UserRepository;

impl UserRepository {
    /// Create a new user
    pub async fn create(
        pool: &PgPool,
        id: Uuid,
        email: &str,
        password_hash: &str,
        full_name: Option<&str>,
    ) -> Result<User, AppError> {
        let now = now();

        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, full_name, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, true)
            RETURNING id, email, full_name, created_at, updated_at, last_login_at, is_active
            "#
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(full_name)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Database(e))
    }

    /// Find user by email (for authentication)
    pub async fn find_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<UserForAuth, AppError> {
        sqlx::query_as::<_, UserForAuth>(
            r#"
            SELECT id, email, password_hash
            FROM users
            WHERE email = $1 AND is_active = true
            "#
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Find user by ID
    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<User, AppError> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, full_name, created_at, updated_at, last_login_at, is_active
            FROM users
            WHERE id = $1 AND is_active = true
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Update last login timestamp
    pub async fn update_last_login(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), AppError> {
        let now = now();

        sqlx::query(
            r#"
            UPDATE users
            SET last_login_at = $1, updated_at = $2
            WHERE id = $3
            "#
        )
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
