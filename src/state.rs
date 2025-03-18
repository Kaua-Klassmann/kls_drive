use std::sync::Arc;

use argon2::Argon2;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: &'static DatabaseConnection,
    pub redis_conn: Arc<&'static Pool<RedisConnectionManager>>,
    pub argon2: Arc<Argon2<'static>>,
}
