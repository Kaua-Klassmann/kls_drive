use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub frontend_url: String,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> AppConfig {
    APP_CONFIG
        .get_or_init(|| {
            let port = env::var("APP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000);

            let frontend_url =
                env::var("FRONTEND_URL").expect("FRONTEND_URL not found at .env file");

            AppConfig { port, frontend_url }
        })
        .clone()
}
