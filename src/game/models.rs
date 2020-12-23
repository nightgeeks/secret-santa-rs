use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};

use super::inner_models::*;

#[derive(Debug)]
pub struct Player<'a> {
    pub id: u32,
    pub name: &'a str,
    pub email: &'a str,
    pub group_id: Option<u32>,
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
    pub gifting_order: Vec<(Player<'game>, Player<'game>)>,
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
