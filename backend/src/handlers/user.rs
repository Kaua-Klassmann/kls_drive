use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bb8_redis::redis::{AsyncCommands, RedisError};
use entity::user;
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{config::redis::get_redis_config, state::AppState};

#[derive(Deserialize, Validate)]
pub struct RegisterUserPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserPayload>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Invalid payload"
            })),
        );
    }

    let db = &state.db_conn;
    let redis = &mut state.redis_conn.get().await.unwrap();
    let argon2 = &state.argon2;

    let cached_user: Result<String, RedisError> =
        redis.get(format!("user:{}", payload.email.clone())).await;

    if cached_user.is_ok() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User already exists"})),
        );
    }

    let user_res = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .one(db)
        .await
        .unwrap();

    if user_res.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User already exists"})),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let activation = Uuid::new_v4();

    let user = user::ActiveModel {
        email: Set(payload.email.clone()),
        password: Set(password_hash.clone()),
        activation: Set(activation),
        ..Default::default()
    };

    let user_res = user::Entity::insert(user).exec(db).await;

    if user_res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create user"})),
        );
    }

    let user_json = serde_json::to_string(&json!({
        "password": password_hash,
        "activation": activation
    }))
    .unwrap();

    let _: String = redis
        .set_ex(
            format!("user:{}", payload.email),
            user_json,
            get_redis_config().ttl,
        )
        .await
        .unwrap();

    (StatusCode::CREATED, Json(json!({})))
}
