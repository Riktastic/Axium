use lettre::{AsyncTransport, Message, message::{header, MultiPart, SinglePart}};
use thiserror::Error;
use std::sync::{Arc, OnceLock};
use tokio::fs;
use html2text;
use html_escape;

use crate::mail::MailerState;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum SmtpError {
    #[error("❌  Environment error: {0}")]
    EnvError(String),
    #[error("❌  SMTP connection error: {0}")]
    ConnectionError(String),
    #[error("❌  SMTP operation error: {0}")]
    OperationError(String),
}

static FOOTER_HTML: OnceLock<Arc<String>> = OnceLock::new();

/// Loads the footer HTML from file (once per process).
async fn get_footer_html() -> Result<Arc<String>, SmtpError> {
    if let Some(f) = FOOTER_HTML.get() {
        Ok(f.clone())
    } else {
        let footer = fs::read_to_string("src/mail/footer.html")
            .await
            .map_err(|e| SmtpError::OperationError(format!("Failed to read footer.html: {}", e)))?;
        Ok(FOOTER_HTML.get().unwrap().clone()) 
    }
}

/// Sends an email with a plain text body and a static HTML footer template as a multipart message.
///
/// # Arguments
/// * `mailer` - The connected AsyncSmtpTransport (from connect_to_mail()).
/// * `from` - The sender's email address.
/// * `to` - The recipient's email address.
/// * `subject` - The email subject.
/// * `body` - The plain text body of the email.
pub async fn send_mail(
    mailer_state: &MailerState,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), SmtpError> {
    let from = &mailer_state.username;
    let mailer = &mailer_state.mailer;

    // Load the footer (cached after first load)
    let html_footer = get_footer_html().await?;

    // Compose plain text part (footer stripped of HTML tags)
    let plain_footer = html2text::from_read(html_footer.as_bytes(), 80)
    .map_err(|e| SmtpError::OperationError(format!("Failed to convert footer HTML to text: {}", e)))?;
    let plain_body = format!("{}\n\n--\n{}", body.trim_end(), plain_footer.trim());

    // Compose HTML part (wrap body and footer in basic HTML)
    let html_body = format!(
        r#"
        <html>
            <body>
                <p>{}</p>
                <hr>
                {}
            </body>
        </html>
        "#,
        html_escape::encode_text(body),
        html_footer
    );

    // Build multipart message
    let multipart = MultiPart::alternative()
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(plain_body)
        )
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_body)
        );

    let email = Message::builder()
        .from(from.parse::<lettre::message::Mailbox>().map_err(|e| SmtpError::OperationError(e.to_string()))?)
        .to(to.parse::<lettre::message::Mailbox>().map_err(|e| SmtpError::OperationError(e.to_string()))?)
        .subject(subject)
        .multipart(multipart)
        .map_err(|e| SmtpError::OperationError(e.to_string()))?;

    mailer.send(email).await.map_err(|e| SmtpError::OperationError(e.to_string()))?;
    Ok(())
}
