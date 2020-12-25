use super::api::*;
use super::models::*;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

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
    fn get_by_id(&self, id: ParticipantId) -> Option<&Participant> {
        self.participants.get(id as usize)
    }

    fn add_with_group(&mut self, name: String, email: String, groups: Groups) -> ParticipantId {
        let id = self.participants.len() as u32;
        let participant = Participant {
            id,
            name,
            email,
            groups,
        };

        self.participants.push(participant);
        return id;
    }

    fn remove(&mut self, _id: ParticipantId) -> Result<(), GameError> {
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
) -> Result<(Sender<'a>, Receiver<'a>), GameError> {
    let sender = senders.pop().ok_or(GameError::UnexpectedGameBehavior)?;
    let position = receivers
        .iter()
        .position(|&receiver| {
            sender.id != receiver.id && sender.groups.intersection(&receiver.groups).count() == 0
        })
        .ok_or(GameError::UnexpectedGameBehavior)?;
    let receiver = receivers.swap_remove(position);
    Result::Ok((sender, receiver))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_with_groups() {
        let mut game = DefaultSecretSanta::default();
        let email = "foo@email.com";
        for gr in 1..10 {
            for s in 1..100 {
                let mut hs = Groups::with_capacity(1);
                hs.insert(gr);
                game.add_with_group(format!("Player {}", s * gr), email.into(), hs);
            }
        }

        let result = game.play();
        assert!(result.is_ok());
        let ok = result.unwrap();
        for (sender, receiver) in ok.gifting_order.iter() {
            assert_ne!(sender.id, receiver.id);
            assert_eq!(sender.groups.intersection(&receiver.groups).count(), 0);
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

    #[test]
    fn get_participant_by_index() {
        let mut game = DefaultSecretSanta::default();
        let player_email = "player@email.com";
        let player_name = "Target player";
        let player_groups: Groups = vec![1, 2, 3].into_iter().collect();

        for s in 0..10 {
            game.add(format!("Player {}", s), "foo@email.com".into());
        }
        let id = game.add_with_group(
            player_name.into(),
            player_email.into(),
            player_groups.clone(),
        );
        for s in 0..10 {
            game.add(format!("Player {}", s), "foo@email.com".into());
        }
        let player = game.get_by_id(id);

        assert!(player.is_some());
        let p = player.unwrap();
        assert_eq!(p.email, player_email);
        assert_eq!(p.name, player_name);
        assert_eq!(p.groups, player_groups);
    }
}
