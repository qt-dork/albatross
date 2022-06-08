use std::fmt;

// The point of this file is to generate most of the game logic so it can be easily called via a functional interface.
use crate::team::Team;
use crate::player::Player;
use crate::java_random::Random;

const TICK: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TopOfInning {
    Top,
    Bottom,
}

pub struct Game {
    pub rng: Random,

    pub teams: EitherOr<Team>,
    pub pitchers: EitherOr<Player>,

    pub day: usize,
    pub start_time: usize,
    pub current_time: usize,
    pub game_log: Vec<GameLog>,
    // pub weather: Weather, // TODO: implement weather

    pub inning: u32,
    pub top: TopOfInning,

    pub batter: Player,
    pub defender: Player,

    pub scores: (f64, f64),
    pub wins: (u32, u32),
    pub balls: u32,
    pub strikes: u32,
    pub outs: u32,
    pub active_team_bat: u32,
    pub inactive_team_bat: u32,

    pub bases: Vec<Player>,
}

impl Game {
    pub fn new(home: Team, away: Team, day: usize, start_time: usize) -> Self {
        let seed: i64 = (day + 4 + home.get_favor() as usize + away.get_favor() as usize) as i64; // Changes in update. TODO: fix this
        Game {
            rng: Random::new(seed),

            teams: EitherOr::new(home, away),
            pitchers: EitherOr::new(Player::default(), Player::default()), // Placeholder
        
            day,
            start_time,
            current_time: 0,
            game_log: Vec::new(),
            // weather: , // TODO: implement weather
        
            inning: 1,
            top: TopOfInning::Top,
        
            batter: Player::default(), // Placeholder
            defender: Player::default(), // Placeholder
        
            scores: (0.0, 0.0),
            wins: (0, 0),
            balls: 0,
            strikes: 0,
            outs: 0,
            active_team_bat: 0,
            inactive_team_bat: 0,
        
            bases: Vec::new(),
        }
    }

    // Figure out a better way to do this
    pub fn start_game(&mut self) {
        let len_home = self.teams.home().get_active_pitchers().len();
        let pitcher_home = self.teams.home()
            .get_active_pitchers()[(self.day - 1 + len_home) % len_home];
        let len_away = self.teams.away().get_active_pitchers().len();
        let pitcher_away = self.teams.away()
            .get_active_pitchers()[(self.day - 1 + len_away) % len_away];
        
        self.pitchers = EitherOr::new(pitcher_home, pitcher_away);
    }

    // Placeholder
    pub fn simulate_game(&mut self) {
        self.start_game();

        self.update("Blay pall!".to_string(), TICK);

        while !self.is_game_over() {
            self.do_inning();
        }

        let message = format!("{} {}, {} {}", self.teams.home().get_name(), self.wins.0, self.teams.away().get_name(), self.wins.1);
    }

    // Placeholder
    pub fn do_inning(&self) {
        let mut message = String::new();
        if self.top == TopOfInning::Top {
            message.push_str("Top");
        } else {
            message.push_str("Bottom");
        }
        message += &format!(" of {}, {} batting. {} pitching.", self.inning, self.teams.batting().get_name(), self.teams.pitching().get_name());
        self.update(message, TICK);

        while !self.is_inning_over() {
            
        }
    }

    // NOTE: This feels off idk. Figure out a better way to do this
    pub fn is_game_over(&self) -> bool {
        self.scores.0 != self.scores.1 && self.inning > 9 && self.top == TopOfInning::Top
    }

    pub fn is_inning_over(&self) -> bool {
        self.outs >= 3
    }

    // pub fn next_pitch(&mut self) -> Vec<GameLog> {
    //     let mut logs = Vec::new();
    //     let mut rng = self.rng.clone();

        
        
    //     self.rng = rng;
    //     logs
    // }

    pub fn update(&mut self, log: String, time: usize) {
        let current_time = self.current_time;
        self.game_log.push(GameLog {
            log,
            time: current_time,
        });
        self.current_time += time;
    }

    pub fn score_as_string(score: f64) -> String {
        let message = String::new();
        if score % 1.0 == 0.0 {
            message.push_str(&format!("{}", score as u32));
        } else {
            message.push_str(&format!("{:.1}", score));
        }
        if score < 0.0 {
            message = format!("({})", message);
        }
        message
    }
}




// This is all data structures stuff. Please ignore.

#[derive(Debug, Clone)]
pub struct GameLog {
    pub log: String,
    pub time: usize,
}

impl GameLog {
    pub fn new(log: String, time: usize) -> Self {
        GameLog {
            log,
            time,
        }
    }
}

impl Default for GameLog {
    fn default() -> Self {
        Self::new(String::new(), 0)
    }
}

#[derive(Debug, Clone)]
pub enum BattingPitching {
    Batting,
    Pitching,
}

/// A data structure that holds two items (e.g. Teams or
/// Players), one for each team, and includes methods to 
/// retrieve the home and away item, as well as the batting
/// and pitching item.
/// 
/// There are getters and setters for every combination.
/// 
/// It stores which item is for the home team, and which is
/// for the away team using the index. Home always gets an
/// index of 0, while away gets an index of 1.
/// 
/// It also stores which team is batting and which is
/// pitching using a flag. The flag is either Batting or
/// Pitching, and it applies to the home team. You can
/// infer the value of the away team, since it's always
/// the opposite of the home team's flag.
#[derive(Debug, Clone)]
pub struct EitherOr<T> {
    pub items: (T, T),
    batting: BattingPitching,
}

impl<T:Clone> EitherOr<T> {
    pub fn new(home: T , away: T) -> Self {
        EitherOr {
            items: (home, away),
            batting: BattingPitching::Pitching,
        }
    }

    pub fn batting(&self) -> T {
        match self.batting {
            BattingPitching::Batting => self.items.0.clone(),
            BattingPitching::Pitching => self.items.1.clone(),
        }
    }

    pub fn set_batting(&mut self, item: T) {
        match self.batting {
            BattingPitching::Batting => self.items.0 = item,
            BattingPitching::Pitching => self.items.1 = item,
        }
    }

    pub fn pitching(&self) -> T {
        match self.batting {
            BattingPitching::Batting => self.items.1.clone(),
            BattingPitching::Pitching => self.items.0.clone(),
        }
    }

    pub fn set_pitching(&mut self, item: T) {
        match self.batting {
            BattingPitching::Batting => self.items.1 = item,
            BattingPitching::Pitching => self.items.0 = item,
        }
    }

    pub fn home(&self) -> T {
        self.items.0.clone()
    }

    pub fn set_home(&mut self, item: T) {
        self.items.0 = item;
    }

    pub fn away(&self) -> T {
        self.items.1.clone()
    }

    pub fn set_away(&mut self, item: T) {
        self.items.1 = item;
    }

    pub fn switch(&mut self) {
        self.batting = match self.batting {
            BattingPitching::Batting => BattingPitching::Pitching,
            BattingPitching::Pitching => BattingPitching::Batting,
        }
    }
}