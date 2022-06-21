// So the goal of this file is to do all team- and player-related JSON parsing.
//use json_library; // for when we decide to import one

// OLD. DON'T USE.

use crate::java_random::Random;
use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Team {
  pub name: String,
  pub location: String,
  pub logo: String,
  pub abbreviation: String,

  lineup: Vec<Player>,
  rotation: Vec<Player>,

  non_losses: u32, // equivalent to alb's actualWins field
  wins: i32,
  losses: i32,

  favor: u32,
  id: u32,
}

impl Team {
  /// Creates a new team with the given name, location, logo, abbreviation, and lineup.
  /// Generates a random lineup, and a random rotation.
  pub fn new(rng: &mut Random, name: String, location: String, logo: String, favor: u32, abbreviation: String) -> Self {
    Team {
      name,
      location,
      logo,
      abbreviation,

      lineup: Team::generate_new_players(rng, 9),
      rotation: Team::generate_new_players(rng, 5),

      non_losses: 0,
      wins: 0,
      losses: 0,

      favor,
      id: 0, // Placeholder
    }
  }

  /// Creates an empty team with the name "team", location "null", and other blank data.
  pub fn empty(rng: &mut Random) -> Self {
    Team::new(rng, "team".to_string(), "null".to_string(), "❓".to_string(), 0, "NULL".to_string())
  }

  pub fn generate_new_players(rng: &mut Random, length: usize) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    for i in 0..length {
      players.push(Player::default(rng));
    }
    players
  }
  
  pub fn get_team_name(&self) -> String {
    if let Some(location) = &self.get_location() {
      return format!("{} {}", self.get_name(), location);
    } else {
      return self.get_name();
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /// Returns the team's location. It's wrapped in an Option, since team locations can apparently be empty?
  pub fn get_location(&self) -> Option<String> {
    if self.location == "" {
      None
    } else {
      Some(self.location.clone())
    }
  }

  pub fn get_active_players(&self) -> Vec<Player> {
    let mut active_players: Vec<Player> = Vec::new();
    for player in &self.lineup {
      active_players.push(player.clone());
    }
    for player in &self.rotation {
      active_players.push(player.clone());
    }
    active_players
  }

  pub fn get_active_pitchers(&self) -> Vec<Player> {
    let mut active_pitchers: Vec<Player> = Vec::new();
    for player in &self.rotation {
      active_pitchers.push(player.clone());
    }
    active_pitchers
  }

  pub fn get_active_batters(&self) -> Vec<Player> {
    let mut active_batters: Vec<Player> = Vec::new();
    for player in &self.lineup {
      active_batters.push(player.clone());
    }
    active_batters
  }

  pub fn get_wins(&self) -> i32 {
    self.wins
  }

  pub fn get_readable_wins(&self) -> String {
    match self.wins {
      win if win == 1 => format!("{} win", win),
      win if win < 0 => format!("({} wins)", win),
      win => format!("{} wins", win),
    }
  }

  pub fn get_losses(&self) -> i32 {
    self.losses
  }

  pub fn get_readable_losses(&self) -> String {
    match self.losses {
      loss if loss == 1 => format!("{} loss", loss),
      loss if loss < 0 => format!("({} losses)", loss),
      loss => format!("{} losses", loss),
    }
  }

  pub fn get_favor(&self) -> u32 {
    self.favor
  }

  pub fn add_win(&mut self) {
    self.wins += 1;
  }

  /// Increases wins by the given amount. Doesn't mean that the team won the game by x amount.
  /// Naming is hard.
  pub fn add_win_by(&mut self, amount: i32) {
    self.wins += amount;
  }

  pub fn add_non_loss(&mut self) {
    self.non_losses += 1;
  }

  pub fn add_loss(&mut self) {
    self.losses += 1;
  }
}

impl PartialEq for Team {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Eq for Team {}