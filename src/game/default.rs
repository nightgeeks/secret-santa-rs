use super::api::*;
use super::inner_models::*;
use super::models::*;
use rand::prelude::SliceRandom;

pub struct DefaultSecretSanta {
    participants: Vec<Participant>,
}

impl Default for DefaultSecretSanta {
    fn default() -> Self {
        DefaultSecretSanta {
            participants: Vec::new(),
        }
    }
}

impl SecretSantaGame for DefaultSecretSanta {
    fn add_with_group(&mut self, name: String, email: String, group: Option<u32>) -> Player {
        let participant = Participant {
            id: self.participants.len() as u32,
            name,
            email,
            group_id: group,
        };

        self.participants.push(participant);
        return Player::from(self.participants.last().unwrap());
    }

    fn remove(&mut self, _player: Player) -> Result<(), GameError> {
        Result::Err(GameError::NotSupported(String::from("remove")))
    }

    fn play(&self) -> Result<GameResult, GameError> {
        let mut result = GameResult::new(self.participants.len());

        let mut senders: Vec<&Participant> = self.participants.iter().collect();
        let mut receivers = senders.clone();

        let mut rng = rand::thread_rng();

        senders.shuffle(&mut rng);
        receivers.shuffle(&mut rng);

        while !senders.is_empty() || !receivers.is_empty() {
            result
                .gifting_order
                .push(find_sender_receiver(&mut senders, &mut receivers)?);
        }
        Result::Ok(result)
    }
}

fn find_sender_receiver<'a>(
    senders: &mut Vec<&'a Participant>,
    receivers: &mut Vec<&'a Participant>,
) -> Result<(Player<'a>, Player<'a>), GameError> {
    let sender = senders.pop().ok_or(GameError::UnexpectedGameBehavior)?;
    let position = receivers
        .iter()
        .position(|&receiver| {
            sender.id != receiver.id
                && (sender.group_id.is_none() || sender.group_id != receiver.group_id)
        })
        .ok_or(GameError::UnexpectedGameBehavior)?;
    let receiver = receivers.swap_remove(position);
    Result::Ok((Player::from(sender), Player::from(receiver)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_with_groups() {
        let mut game = DefaultSecretSanta::default();
        let email = "foo@email.com";
        for gr in 0..10 {
            for s in 0..100 {
                game.add_with_group(format!("Player {}", s * gr), email.into(), Some(gr));
            }
        }

        let result = game.play();
        assert!(result.is_ok());
        let ok = result.unwrap();
        for (sender, receiver) in ok.gifting_order.iter() {
            assert_ne!(sender.id, receiver.id);
            assert_ne!(sender.group_id, receiver.group_id);
        }
    }

    #[test]
    fn play_without_groups() {
        let mut game = DefaultSecretSanta::default();
        let email = "foo@email.com";
        for s in 0..10_000 {
            game.add(format!("Player {}", s), email.into());
        }
        let result = game.play();
        assert!(result.is_ok());
        let ok = result.unwrap();
        for (sender, receiver) in ok.gifting_order.iter() {
            assert_ne!(sender.id, receiver.id);
        }
    }
}
