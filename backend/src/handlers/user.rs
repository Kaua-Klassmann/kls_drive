use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use bb8_redis::redis::{AsyncCommands, RedisError};
use entity::user;
use lettre::{AsyncTransport, Message};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::{app::get_app_config, email::get_email_config, redis::get_redis_config},
    state::AppState,
};

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
    let email_mailer = &state.email_mailer;
    let argon2 = &state.argon2;

    let cached_user: Result<String, RedisError> = redis
        .get(format!("user_exists:{}", payload.email.clone()))
        .await;

    if cached_user.is_ok() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User already exists"})),
        );
    }

    let user_res = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .one(db)
        .await;

    if user_res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed on server"
            })),
        );
    }

    if user_res.unwrap().is_some() {
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
        activation: Set(Some(activation)),
        ..Default::default()
    };

    let user_res = user::Entity::insert(user).exec(db).await;

    if user_res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create user"})),
        );
    }

    let frontend_url = get_app_config().frontend_url.clone();

    let email = Message::builder()
        .from(format!("<{}>", get_email_config().email).parse().unwrap())
        .to(format!("<{}>", payload.email.clone()).parse().unwrap())
        .subject("Activate your account")
        .body(format!("{}/activate/{}", frontend_url, activation))
        .unwrap();

    email_mailer.send(email).await.unwrap();

    let _: String = redis
        .set_ex(
            format!("user_exists:{}", payload.email),
            "".to_string(),
            get_redis_config().ttl,
        )
        .await
        .unwrap();

    let _: String = redis
        .set_ex(
            format!("activate_user:{}", activation),
            payload.email,
            get_redis_config().ttl,
        )
        .await
        .unwrap();

    (StatusCode::CREATED, Json(json!({})))
}
