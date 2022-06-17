use crate::utils::EntityId;
use crate::game::Game;

#[derive(Clone, Debug)]
pub struct League {
  pub players: Players,
  pub teams: Teams,
  pub games: Games,
}

impl League {
  pub fn new(players: Players, teams: Teams, games: Games) -> Self {
    League {
      players,
      teams,
      games,
    }
  }

  pub fn empty() -> Self {
    League {
      players: Players::empty(),
      teams: Teams::empty(),
      games: Games::empty(),
    }
  }

  pub fn find_by_name
}