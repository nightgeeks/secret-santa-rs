use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

// TODO - make external
pub const SMTP_USERNAME: &str = "heartshaped20box@gmail.com";
pub const APP_PASSWORD: &str = "oqakbbxzamltptcc";
pub const SMTP_SERVER: &str = "smtp.gmail.com";

#[derive(Clone)]
pub struct Client {
    pub(crate) name: String,
    pub(crate) email: String,
}

impl Client {
    pub fn new(name: String, email: String) -> Self {
        Client { name, email }
    }
}

impl From<(String, String)> for Client {
    fn from(pair: (String, String)) -> Self {
        Client::new(pair.0, pair.1)
    }
}

impl Into<String> for Client {
    fn into(self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

impl Into<Mailbox> for Client {
    fn into(self) -> Mailbox {
        let s: String = self.into();
        s.parse().unwrap()
    }
}

pub struct Email {
    receiver: Client,
    subject: String,
    body: String,
}

impl Email {
    pub fn new(receiver: Client, subject: String, body: String) -> Self {
        Email {
            receiver,
            subject,
            body,
        }
    }
}

pub struct EmailClient {
    sender: Client,
    mailer: SmtpTransport,
}

impl EmailClient {
    pub fn new(email: String, password: String, sender_name: String, smtp_server: String) -> Self {
        EmailClient {
            sender: (sender_name, email.clone()).into(),
            mailer: SmtpTransport::relay(smtp_server.as_str())
                .unwrap()
                .credentials((email, password).into())
                .build(),
        }
    }

    pub fn send(&self, email: Email) -> Result<(), lettre::transport::smtp::Error> {
        let message = Message::builder()
            .from(self.sender.clone().into())
            .to(email.receiver.clone().into())
            .subject(email.subject)
            .body(email.body)
            .unwrap();

        self.mailer.send(&message).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_email() {
        let client = EmailClient::new(
            String::from(SMTP_USERNAME),
            String::from(APP_PASSWORD),
            "Secret Santa".into(),
            String::from(SMTP_SERVER),
        );

        let email = Email::new(
            Client::new("Osoleses".to_string(), "qqvsorrow@gmail.com".to_string()),
            "Secret Santa".to_string(),
            "Your target is: 1234567890".to_string(),
        );

        let result = client.send(email);
        assert!(result.is_ok());
    }
}
