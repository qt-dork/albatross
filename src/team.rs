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

}
