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
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};
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
            format!("activate_user:{}", activation),
            user_res.unwrap().last_insert_id,
            get_redis_config().ttl,
        )
        .await
        .unwrap();

    (StatusCode::CREATED, Json(json!({})))
}

#[derive(FromQueryResult)]
struct UserWithId {
    id: u32,
}

pub async fn activate_user(
    State(state): State<AppState>,
    Path(activate_code): Path<Uuid>,
) -> impl IntoResponse {
    let db = &state.db_conn;
    let redis = &mut state.redis_conn.get().await.unwrap();

    let user_id: u32;

    let cached_user: Result<String, RedisError> =
        redis.get(format!("activate_user:{}", activate_code)).await;

    if cached_user.is_ok() {
        user_id = cached_user.unwrap().parse().unwrap();

        let _: String = redis
            .del(format!("activate_user:{}", activate_code))
            .await
            .unwrap();
    } else {
        let user_result_db = user::Entity::find()
            .select_only()
            .columns([user::Column::Id])
            .filter(user::Column::Activation.eq(activate_code))
            .into_model::<UserWithId>()
            .one(db)
            .await;

        if user_result_db.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to find user"
                })),
            );
        }

        let user_result = user_result_db.unwrap();

        if user_result.is_none() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "User not found"
                })),
            );
        }

        user_id = user_result.unwrap().id;
    }

    let user_model = user::ActiveModel {
        id: Set(user_id),
        activation: Set(None),
        ..Default::default()
    };

    let update_user_result = user::Entity::update(user_model)
        .filter(user::Column::Id.eq(user_id))
        .exec(db)
        .await;

    if update_user_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update user"
            })),
        );
    }

    (StatusCode::OK, Json(json!({})))
}
