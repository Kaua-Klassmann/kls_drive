use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct JwtOpts {
    pub secret: String,
}

static JWT_OPTS: OnceLock<JwtOpts> = OnceLock::new();

pub fn get_jwt_opts() -> JwtOpts {
    JWT_OPTS
        .get_or_init(|| {
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET not found at .env file");

            JwtOpts { secret }
        })
        .clone()
}
