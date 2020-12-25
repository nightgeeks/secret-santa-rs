use super::models::*;

pub trait SecretSantaGame {
    fn get_by_id(&self, id: ParticipantId) -> Option<&Participant>;
    fn add_with_group(&mut self, name: String, email: String, group: Groups) -> ParticipantId;
    fn remove(&mut self, id: ParticipantId) -> Result<(), GameError>;
    fn play(&self) -> Result<GameResult, GameError>;

    fn add(&mut self, name: String, email: String) -> ParticipantId {
        self.add_with_group(name, email, Groups::new())
    }
}
