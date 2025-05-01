// Module declarations
pub mod connect;
pub mod send;

use lettre::{AsyncSmtpTransport, Tokio1Executor};

#[derive(Clone, Debug)]
pub struct MailerState {
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
    pub username: String, // This will be your "from" address
}