use bb8::PooledConnection;
use bb8_redis::{
    RedisConnectionManager,
    redis::{AsyncCommands, RedisError},
};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config;

#[derive(Deserialize, Serialize, FromQueryResult)]
pub struct User {
    pub user_id: u32,
    pub password: String,
    pub activated: bool,
}

pub async fn set_user(
    redis: &mut PooledConnection<'_, RedisConnectionManager>,
    user_id: u32,
    email: &String,
    password: &String,
    activated: bool,
) -> Result<(), RedisError> {
    let json_data = serde_json::to_string(&User {
        user_id,
        password: password.to_owned(),
        activated,
    })
    .unwrap();

    redis
        .set_ex(
            format!("user:{}", email),
            json_data,
            config::redis::get_redis_config().ttl,
        )
        .await
}

pub async fn get_user(
    redis: &mut PooledConnection<'_, RedisConnectionManager>,
    email: &String,
) -> Result<User, RedisError> {
    let resp: Result<String, RedisError> = redis.get(format!("user:{}", email)).await;

    if resp.is_err() {
        return Err(resp.err().unwrap());
    }

    Ok(serde_json::from_str(resp.unwrap().as_str()).unwrap())
}

pub async fn delete_user(
    redis: &mut PooledConnection<'_, RedisConnectionManager>,
    email: &String,
) -> Result<(), RedisError> {
    redis.del(format!("user:{}", email)).await
}

#[derive(Deserialize, Serialize)]
pub struct ActivateUser {
    pub user_id: u32,
}

pub async fn set_activate_user(
    redis: &mut PooledConnection<'_, RedisConnectionManager>,
    activation_code: &Uuid,
    user_id: u32,
) -> Result<(), RedisError> {
    let json_data = serde_json::to_string(&ActivateUser { user_id }).unwrap();

    redis
        .set_ex(
            format!("activate_user:{}", activation_code),
            json_data,
            config::redis::get_redis_config().ttl,
        )
        .await
}

pub async fn get_activate_user(
    redis: &mut PooledConnection<'_, RedisConnectionManager>,
    activate_code: &Uuid,
) -> Result<ActivateUser, RedisError> {
    let resp: Result<String, RedisError> =
        redis.get(format!("activate_user:{}", activate_code)).await;

    if resp.is_err() {
        return Err(resp.err().unwrap());
    }

    Ok(serde_json::from_str(resp.unwrap().as_str()).unwrap())
}

pub async fn del_activate_user(
    redis: &mut PooledConnection<'_, RedisConnectionManager>,
    activate_code: &Uuid,
) -> Result<(), RedisError> {
    redis.del(format!("activate_user:{}", activate_code)).await
}
