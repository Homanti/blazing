use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use blazing_models::{RegisterRequest, AuthResponse, AppError, User, LoginRequest};

pub struct AuthService {
    pub db_pool: PgPool,
    pub jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

impl AuthService {
    pub fn new(db_pool: PgPool, jwt_secret: String) -> Self {
        Self { db_pool, jwt_secret }
    }

    fn create_jwt(&self, user_id: Uuid) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .ok_or(AppError::Internal("Failed to calculate expiration".to_string()))?
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        ).map_err(|e| AppError::Internal(format!("Failed to encode Token: {}", e)))?;

        Ok(token)
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, AppError> {
        let password_hash = hash(&request.password, DEFAULT_COST).map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?;

        let existing = sqlx::query!(
            r#"
            SELECT
                email as "email!",
                username as "username!"
            FROM users
            WHERE email = $1 OR username = $2
            "#, request.email, request.username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(user) = existing {
            if user.email == request.email {
                return Err(AppError::BadRequest("Email already exists".to_string()));
            }
            if user.username == request.username {
                return Err(AppError::BadRequest("Username already exists".to_string()));
            }
        }

        let user = sqlx::query_as!(User,
            r#"
                INSERT INTO users (username, email, password_hash)
                VALUES ($1, $2, $3)
                RETURNING *
            "#, request.username, request.email, password_hash
        )
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to create user: {}", e)))?;

        let token = self.create_jwt(user.id)?;

        Ok(AuthResponse {
            user,
            token,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = sqlx::query_as!(User, r#"
            SELECT id, username, email, password_hash, avatar_url, created_at, updated_at
            FROM users WHERE email=$1"#, request.email
        )
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("Error during login: {}", e.to_string())))?;

        let user = user.ok_or(AppError::Unauthorized("User not found".to_string()))?;

        let password_is_valid = bcrypt::verify(&request.password, &user.password_hash).map_err(|e| AppError::Internal(format!("Failed to verify password: {}", e.to_string())))?;

        if !password_is_valid {
            return Err(AppError::BadRequest("Invalid email or password".to_string()));
        }

        let token = self.create_jwt(user.id)?;

        Ok(AuthResponse {
            user,
            token,
        })
    }
}