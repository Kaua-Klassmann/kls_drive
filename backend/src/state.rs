use std::sync::Arc;

use argon2::Argon2;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub redis_conn: Arc<Pool<RedisConnectionManager>>,
    pub email_mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    pub argon2: Arc<Argon2<'static>>,
}
