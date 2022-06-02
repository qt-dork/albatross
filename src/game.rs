// The point of this file is to generate most of the game logic so it can be easily called via a functional interface.
use crate::team::Team;
use crate::player::Player;
use crate::java_random::Random;

pub struct Game {
    pub rng: Random,

    pub inning: u32,
    pub top: bool,

    pub home: Team,
    pub score_home: i32,
    pub away: Team,
    pub score_away: i32,
    
    pub pitcher_home: Player,
    pub pitcher_away: Player,
    pub batter: Player,

    pub bases: Vec<Player>,

    pub outs: i32,
    pub strikes: i32,
    pub balls: i32,

    pub day: usize,
    pub season: u32,

    pub weather: String, // Make weather stuff later
}

impl Game {
    pub fn new(home: Team, away: Team, day: usize, season: u32) -> Self {
        let seed: i64 = (day + 4 + home.get_favor() as usize + away.get_favor() as usize) as i64;
        Game {
            rng: Random::new(seed),

            inning: 1,
            top: true,

            home,
            score_home: 0,
            away,
            score_away: 0,

            pitcher_home: Player::new(),
            pitcher_away: Player::new(),
            batter: Player::new(),

            bases: Vec::new(),

            outs: 0,
            strikes: 0,
            balls: 0,

            day,
            season,

            weather: "Test".to_string(),
        }
    }

    pub fn next_pitch(&mut self)->GameLog {
        let mut logs = GameLog::new();
        let mut rng = self.rng.clone();

        self.pitcher_home = self.home.get_active_pitchers()[(self.day - 1 + self.home.get_active_pitchers().len()) % self.home.get_active_pitchers().len()].clone();
        self.batter = self.away.get_active_batters()[0].clone();

        // Run pitch
        let pitch_value = self.pitcher_home.aggression + rng.next_f64() * 5.0;
        let bat_value = self.batter.anti_blasedness + rng.next_f64() * 5.0;

        let pitch_result = pitch_value > bat_value;

        println!("pitch_value: {}\nbat_value: {}", pitch_value, bat_value);

        if pitch_result {
            logs.push(format!("{} pitches well!", self.pitcher_home.get_name()));
        } else {
            logs.push(format!("{} bats well!", self.batter.get_name()));
            logs.push(format!("{} pitches poorly!", self.pitcher_home.get_name()));
        }
        self.rng = rng;
        logs
    }
}

pub struct GameLog(Vec<String>);
impl GameLog {
    pub fn new() -> Self {
        GameLog(Vec::new())
    }

    pub fn push(&mut self, message: String) {
        self.0.push(message);
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.0.clone()
    }
}