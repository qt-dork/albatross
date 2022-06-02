// So the goal of this file is to do all team- and player-related JSON parsing.
//use json_library; // for when we decide to import one

use crate::java_random::Random;
use crate::player::Player;

pub struct Team {
  name: String,
  location: String,
  logo: String,
  abbreviation: String,
  lineup: Vec<Player>,
  rotation: Vec<Player>,

  non_losses: u32, // equivalent to alb's actualWins field
  wins: u32,
  losses: u32,

  favor: u32,
  id: u32,
}

impl Team {
  /// Creates a new team with the given name, location, logo, abbreviation, and lineup.
  /// Generates a random lineup, and a random rotation.
  pub fn new(name: String, location: String, logo: String, favor: u32, abbreviation: String) {

  }

  // pub fn from_json(json: &json_library::Team) -> Team {
  //   let mut lineup = Vec::new();
  //   let mut rotation = Vec::new();

  //   for player in json.lineup.iter() {
  //     lineup.push(Player::from(player));
  //   }

  //   for player in json.rotation.iter() {
  //     rotation.push(Player::from(player));
  //   }

  //   Team {
  //     name: json.name.clone(),
  //     location: json.location.clone(),
  //     logo: json.logo.clone(),
  //     abbreviation: json.abbreviation.clone(),
  //     lineup: lineup,
  //     rotation: rotation,
  //     non_losses: json.actualWins,
  //     wins: json.wins,
  //     losses: json.losses,
  //     favor: json.favor,
  //     id: json.id,
  //   }
  // }
}