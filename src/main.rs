use std::time::{Duration, SystemTime};

use java_random::Random;

fn main() {
    let mut rng = Random::new(-4);

    let mut game = game::Game::new(
        team::Team::empty(&mut rng),
        team::Team::empty(&mut rng),
        4,
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
    );
    game.simulate_game();
    game.play_logs();
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
