pub mod email;
pub mod game;

use email::*;
use game::*;

fn some() {
    let game = new_game();
}

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
