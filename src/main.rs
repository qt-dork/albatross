use std::time::{Duration, SystemTime};

use albatross::league::League;
// use albatross::util::name_generator::name_generator::NameGenerator;
// use albatross::util::rng::Rand32;

fn main() {
    let mut league = League::new();
    league.initialize_teams();
    println!("{:#?}", league.players.name);
}

// Thought on structure
// Create a Game struct that has all the information it needs to run
// Give the struct a .next() method that returns the next state of the game
// Give the struct a .is_over() method that returns true if the game is over
// Iterate through each game and increment the progress of the game until the game is over

// How the program works:
// Loads data for games from the JSON database
// Runs a game for each game in the database
// Each tick's results will be added to a queue of messages
// They're printed at the end of the tick
// When all games are over, the program ends
// Then the results of the games are added to the database and updated

// struct Game {
//     rng: u32, // Change later. Needs to be rng,
//     game_logs: Vec<String>,
// }

// impl Game {
//     fn new() -> Game {
//         // Instantiate game
//     }

//     fn next(&self) {
//         // run next state of game
//         let mut next = self.clone();

//         // Run loop

//         self = next;
//     }

//     fn is_over(&self) -> bool {
//         // Check if game is over
//     }

//     fn get_logs(&self) -> Vec<String> {
//         // Get logs
//     }
// }
