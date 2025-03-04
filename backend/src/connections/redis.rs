use std::str::FromStr;

use bb8::Pool;
use bb8_redis::{RedisConnectionManager, redis::ConnectionInfo};
use tokio::sync::OnceCell;

use crate::config::redis::get_redis_config;

static REDIS: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn get_redis_connection() -> Pool<RedisConnectionManager> {
    REDIS
        .get_or_init(|| async {
            let redis_config = get_redis_config();

            let connection_info = ConnectionInfo::from_str(&redis_config.url).unwrap();

            let manager = RedisConnectionManager::new(connection_info)
                .expect("Failed to create redis manager");

            let pool = Pool::builder()
                .max_size(redis_config.max_connections)
                .build(manager)
                .await
                .expect("Failed to create redis pool");

            pool
        })
        .await
        .clone()
}
