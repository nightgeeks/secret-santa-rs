mod secret_santa {
    use core::fmt;
    use rand::prelude::SliceRandom;
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Clone)]
    struct Participant {
        id: u32,
        name: String,
        email: String,
        // TODO - add set of groups ^_^
        group_id: Option<u32>,
    }

    #[derive(Debug)]
    pub struct Player<'a> {
        id: u32,
        pub name: &'a str,
        pub email: &'a str,
        group_id: Option<u32>,
    }

    impl<'a> From<&'a Participant> for Player<'a> {
        fn from(participant: &'a Participant) -> Self {
            Player {
                id: participant.id,
                name: participant.name.as_str(),
                email: participant.email.as_str(),
                group_id: participant.group_id,
            }
        }
    }

    pub struct GameResult<'game> {
        gifting_order: Vec<(Player<'game>, Player<'game>)>,
    }

    impl GameResult<'_> {
        fn new(size: usize) -> Self {
            GameResult {
                gifting_order: Vec::with_capacity(size),
            }
        }
    }

    struct DefaultSecretSanta {
        participants: Vec<Participant>,
    }

    trait SecretSantaGame {
        fn add_with_group(&mut self, name: String, email: String, group: Option<u32>) -> Player;
        fn remove(&mut self, player: Player) -> Result<(), GameError>;
        fn play(&self) -> Result<GameResult, GameError>;

        fn add(&mut self, name: String, email: String) -> Player {
            return self.add_with_group(name, email, None);
        }
    }

    impl DefaultSecretSanta {
        fn new() -> Self {
            return DefaultSecretSanta {
                participants: Vec::new(),
            };
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
            return Result::Err(GameError::NotSupported(String::from("remove")));
        }

        fn play(&self) -> Result<GameResult, GameError> {
            let mut result = GameResult::new(self.participants.len());

            let mut senders: Vec<&Participant> = self.participants.iter().map(|p| p).collect();
            let mut receivers = senders.clone();

            let mut rng = rand::thread_rng();

            senders.shuffle(&mut rng);
            receivers.shuffle(&mut rng);

            while !senders.is_empty() || !receivers.is_empty() {
                result.gifting_order.push(find_sender_receiver(&mut senders, &mut receivers)?);
            }
            return Result::Ok(result);
        }
    }

    fn find_sender_receiver<'a>(
        senders: &mut Vec<&'a Participant>,
        receivers: &mut Vec<&'a Participant>,
    ) -> Result<(Player<'a>, Player<'a>), GameError> {
        let sender = senders.pop().ok_or(GameError::UnexpectedGameBehavior)?;
        let position = receivers.iter()
            .position(|&receiver|
                sender.id != receiver.id && (sender.group_id.is_none() || sender.group_id != receiver.group_id))
            .ok_or(GameError::UnexpectedGameBehavior)?;
        let receiver = receivers.swap_remove(position);
        return Result::Ok((Player::from(sender), Player::from(receiver)));
    }

    #[derive(Debug)]
    enum GameError {
        NotSupported(String),
        UnexpectedGameBehavior,
    }

    impl Display for GameError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            match self {
                GameError::NotSupported(msg) => {
                    write!(f, "Operation is not supported: {}", msg)
                }
                GameError::UnexpectedGameBehavior => {
                    write!(f, "Unexpected Game Behavior")
                }
            }
        }
    }

    impl Error for GameError {}

    #[cfg(test)]
    mod tests {
        use super::*;
        // use test::bench::benchmark;
        // use test::Bencher;

        #[test]
        fn play_with_groups() {
            let mut game = DefaultSecretSanta::new();
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
            let mut game = DefaultSecretSanta::new();
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

        // #[bench]
        // fn bench_add_two(b: &mut Bencher) {
        //     let mut game = DefaultSecretSanta::new();
        //     let email = "foo@email.com";
        //     for s in 0..10_000 {
        //         game.add(format!("Player {}", s), email.into());
        //     }
        //
        //     b.iter(|| {
        //         let result = game.play();
        //         assert!(result.is_ok());
        //         let ok = result.unwrap();
        //     });
        // }
    }
}
