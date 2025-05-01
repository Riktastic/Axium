use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use thiserror::Error;

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

pub async fn connect_to_mail() -> Result<AsyncSmtpTransport<Tokio1Executor>, SmtpError> {
    let smtp_server = std::env::var("MAIL_SERVER").map_err(|e| SmtpError::EnvError(e.to_string()))?;
    let smtp_port = std::env::var("MAIL_PORT").unwrap_or_else(|_| "587".to_string());
    let smtp_user = std::env::var("MAIL_USER").map_err(|e| SmtpError::EnvError(e.to_string()))?;
    let smtp_pass = std::env::var("MAIL_PASS").map_err(|e| SmtpError::EnvError(e.to_string()))?;

    let creds = Credentials::new(smtp_user, smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)
        .map_err(|e| SmtpError::ConnectionError(e.to_string()))?
        .port(smtp_port.parse().unwrap_or(587))
        .credentials(creds)
        .build();

    Ok(mailer)
}