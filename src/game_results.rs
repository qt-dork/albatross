use uuid::{Uuid, Builder};

use crate::comp::*;
use crate::types::GameKind;
use crate::messaging::{Message, MessageLog};
use crate::weather::Weather;


#[derive(Clone, Debug, Default)]
pub struct GameResult {
    pub uuid: Uuid,
    pub season: usize,
    pub day: u32,

    pub kind: GameKind,
    pub away: TeamId,
    pub home: TeamId,
    pub weather: Weather,
    pub logs: MessageLog,
}

impl GameResult {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }
}

impl PartialEq for GameResult {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}