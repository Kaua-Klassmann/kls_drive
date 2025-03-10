use lettre::{AsyncTransport, Message, transport::smtp::Error};

use crate::{config, connections};

pub async fn send_email(to: String, subject: String, body: String) -> Result<(), Error> {
    let mailer_send = connections::email::get_email_mailer();
    let email_config = config::email::get_email_config();

    let email = Message::builder()
        .from(format!("<{}>", email_config.email).parse().unwrap())
        .to(format!("<{}>", to).parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    tokio::spawn(async { mailer_send.send(email).await });

    Ok(())
}
