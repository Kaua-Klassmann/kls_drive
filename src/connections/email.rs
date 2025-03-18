use std::sync::OnceLock;

use lettre::{AsyncSmtpTransport, Tokio1Executor, transport::smtp::authentication::Credentials};

use crate::config;

static EMAIL_MAILER: OnceLock<AsyncSmtpTransport<Tokio1Executor>> = OnceLock::new();

pub fn get_email_mailer() -> &'static AsyncSmtpTransport<Tokio1Executor> {
    EMAIL_MAILER.get_or_init(|| {
        let email_config = config::email::get_email_config();

        let credentials = Credentials::new(email_config.email, email_config.password);

        AsyncSmtpTransport::<Tokio1Executor>::relay(&email_config.smtp)
            .expect("Failed to create email mailer")
            .credentials(credentials)
            .build()
    })
}
