use std::env;

pub struct EmailConfig {
    pub email: String,
    pub password: String,
    pub smtp: String,
}

pub fn get_email_config() -> EmailConfig {
    let email = env::var("MAILER_EMAIL").expect("MAILER_EMAIL not found at .env file");
    let password = env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD not found at .env file");
    let smtp = env::var("MAILER_SMTP").expect("MAILER_SMTP not found at .env file");

    EmailConfig {
        email,
        password,
        smtp,
    }
}
