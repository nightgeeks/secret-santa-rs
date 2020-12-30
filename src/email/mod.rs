use std::error::Error;

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{Mailbox, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;

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
        let part = SinglePart::builder()
            .content_type(ContentType::html())
            .body(email.body);
        let message = Message::builder()
            .from(self.sender.clone().into())
            .to(email.receiver.clone().into())
            .subject(email.subject)
            .singlepart(part)
            .unwrap();

        self.mailer.send(&message).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn send_email() {
        let mut html_file = File::open("D:\\Development\\Projects\\pets\\secret_santa\\index.html").unwrap();
        let mut html = String::new();
        html_file.read_to_string(&mut html);


        let client = EmailClient::new(
            String::from(SMTP_USERNAME),
            String::from(APP_PASSWORD),
            "Secret Santa".into(),
            String::from(SMTP_SERVER),
        );

        let email = Email::new(
            Client::new("Osoleses".to_string(), "qqvsorrow@gmail.com".to_string()),
            "Secret Santa".to_string(),
            html,
        );

        let result = client.send(email);
        assert!(result.is_ok());
    }
}
