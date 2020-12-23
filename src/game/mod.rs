pub mod api;
mod default;
mod inner_models;
pub mod models;

pub use api::SecretSantaGame;
pub use default::DefaultSecretSanta;
pub use models::*;

pub fn new_game() -> DefaultSecretSanta {
    DefaultSecretSanta::default()
}
