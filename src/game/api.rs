use super::models::*;

pub trait SecretSantaGame {
    fn add_with_group(&mut self, name: String, email: String, group: Option<u32>) -> Player;
    fn remove(&mut self, player: Player) -> Result<(), GameError>;
    fn play(&self) -> Result<GameResult, GameError>;

    fn add(&mut self, name: String, email: String) -> Player {
        self.add_with_group(name, email, None)
    }
}
