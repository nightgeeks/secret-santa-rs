use core::fmt;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};


pub type ParticipantId = u32;
pub type GroupId = u32;
pub type Groups = HashSet<GroupId>;


#[derive(Clone, Debug)]
pub struct Participant {
    pub id: ParticipantId,
    pub name: String,
    pub email: String,
    pub groups: Groups,
}

pub type Sender<'t> = &'t Participant;
pub type Receiver<'t> = &'t Participant;

pub struct GameResult<'game> {
    pub gifting_order: Vec<(Sender<'game>, Receiver<'game>)>,
}

impl GameResult<'_> {
    pub fn new(size: usize) -> Self {
        GameResult {
            gifting_order: Vec::with_capacity(size),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
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
