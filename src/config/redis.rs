use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub ttl: u64,
}

static REDIS_CONFIG: OnceLock<RedisConfig> = OnceLock::new();

pub fn get_redis_config() -> RedisConfig {
    REDIS_CONFIG
        .get_or_init(|| {
            let url = env::var("REDIS_URL").expect("REDIS_URL not found at .env file");
            let max_connections = env::var("REDIS_MAX_CONNECTIONS")
                .expect("REDIS_MAX_CONNECTIONS not found at .env file")
                .parse::<u32>()
                .expect("REDIS_MAX_CONNECTIONS must be a number");
            let ttl = env::var("REDIS_TTL")
                .expect("REDIS_TTL not found at .env file")
                .parse::<u64>()
                .expect("REDIS_TTL must be a number");

            RedisConfig {
                url,
                max_connections,
                ttl,
            }
        })
        .clone()
}
