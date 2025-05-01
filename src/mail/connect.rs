use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor, Message, AsyncTransport};
use thiserror::Error;

use crate::core::config::{get_env, get_env_with_default};
use crate::mail::MailerState;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum SmtpError {
    #[error("❌  Environment error: {0}")]
    EnvError(String),
    #[error("❌  Mail connection error: {0}")]
    ConnectionError(String),
    #[error("❌  Mail operation error: {0}")]
    OperationError(String),
}

pub async fn connect_to_mail() -> Result<MailerState, SmtpError> {
    let smtp_server = get_env("MAIL_SERVER");
    let smtp_port = get_env_with_default("MAIL_PORT", "587");
    let smtp_user = get_env("MAIL_USER");
    let smtp_pass = get_env("MAIL_PASS");

    let creds = Credentials::new(smtp_user.clone(), smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)
        .map_err(|e| SmtpError::ConnectionError(e.to_string()))?
        .port(smtp_port.parse().unwrap_or(587))
        .credentials(creds)
        .build();

    // Send a test email to the `MAIL_USER` address
    let test_email = Message::builder()
        .from(smtp_user.parse::<lettre::message::Mailbox>().map_err(|e| SmtpError::OperationError(e.to_string()))?)
        .to(smtp_user.parse::<lettre::message::Mailbox>().map_err(|e| SmtpError::OperationError(e.to_string()))?)
        .subject("SMTP Test")
        .body("This is a test email sent from the Axium SMTP connection validation. Axium will sent a verification mail during each startup to test if email is working properly. You can ignore this mail.".to_string())
        .map_err(|e| SmtpError::OperationError(e.to_string()))?;

    // Send the test email and validate the connection
    match mailer.send(test_email).await {
        Ok(_) => {
            Ok(MailerState {
                mailer,
                username: smtp_user,
            })
        }
        Err(e) => Err(SmtpError::OperationError(format!("Failed to send test email: {}", e))),
    }
}