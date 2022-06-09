use std::fmt;

use std::time::{Duration, SystemTime};

// The point of this file is to generate most of the game logic so it can be easily called via a functional interface.
use crate::team::Team;
use crate::player::Player;
use crate::java_random::Random;

const TICK: u64 = 100;

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
    pub start_time: u64, // NOTE: I feel like this could be collapsed into a single field? Like a start_time current_time struct.
    pub current_time: u64,
    pub game_log: Vec<GameLog>,
    // pub weather: Weather, // TODO: implement weather

    pub inning: u32,
    pub top: TopOfInning,

    pub batter: Player,
    pub defender: Player,

    pub scores: (f64, f64),
    pub wins: (i32, i32),
    pub balls: i32,
    pub strikes: i32,
    pub outs: i32,
    pub active_team_bat: u32, // Maybe this should be EitherOr?
    pub inactive_team_bat: u32, // Wait no it can't. It's a u32.

    pub bases: Vec<Player>,
}

impl Game {
    pub fn new(home: Team, away: Team, day: usize, start_time: u64) -> Self {
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

        let sys_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        
        let cur_time = sys_time.as_secs();

        while !self.is_game_over() {
            self.do_inning();
        }

        let message = format!("{} {}, {} {}", self.teams.home().get_name(), self.wins.0, self.teams.away().get_name(), self.wins.1);
        self.update(message, TICK);
        self.update("\nGame Over.".to_string(), TICK);
        self.give_wins();
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
            self.clear_bases();
            self.set_next_batter();
            while !self.has_struck_out() {
                self.do_steals();
                self.do_pitch();
            }
            self.strikes = 0;
            self.balls = 0;
            self.outs += 1;
            self.update(format!("[Out {}]", self.outs), 0);
        }

        // Do end of inning stuff
        self.outs = 0;
        match self.top {
            TopOfInning::Top => self.top = TopOfInning::Bottom,
            TopOfInning::Bottom => {
                self.top = TopOfInning::Top;
                self.update(format!("Inning {} is now an Outing."), TICK);
                self.print_score(); // NOTE: I don't like this

                self.inning += 1;
            },
        }

        // Switch sides
        self.teams.switch_sides();
        self.pitchers.switch_sides();

        // Switch out teams' batting positions
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

    pub fn update(&mut self, log: String, time: u64) {
        let current_time = self.current_time; // See if there's a better way to handle current time?
        self.game_log.push(GameLog {
            log,
            time: current_time,
        });
        self.current_time += time;
    }

    pub fn get_game_name(&self) -> String {
        format!(
            "{}, {} vs. {}, Day {}", 
            self.teams.home().get_name(), // TODO: Make this the current weather once that is implemented
            self.teams.away().get_name(),
            self.teams.home().get_name(), 
            self.day
        )
    }

    // Precondition: simulate_game() has been called already
    pub fn is_live(&self) -> bool {
        self.current_time == self.start_time
    }

    pub fn get_random_defender(&mut self) -> Player {
        let players = self.teams.pitching().get_active_batters();
        let random_number = (self.rng.next_f64() * players.len() as f64) as usize;

        players[random_number]
    }

    /// Returns true if the batter has struck out.
    /// 
    /// NOTE: This will need to be updated to be variable
    /// in the future if features like the fourth base are
    /// added
    pub fn has_struck_out(&self) -> bool {
        self.strikes >= 3
    }

    pub fn format_balls_strikes(&self) -> String {
        let b = self.balls.to_string();
        let s = self.strikes.to_string();

        if self.balls < 0 {
            b = format!("(-{})", self.balls.abs());
        }
        if self.strikes < 0 {
            s = format!("(-{})", self.strikes.abs());
        }
        return format!("{}-{}", b, s);
    }

    pub fn clear_bases(&mut self) {
        self.bases.clear();
    }
}


// Game function but not part of game struct
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


// This is all data structures stuff. Please ignore.

#[derive(Debug, Clone)]
pub struct GameLog {
    pub log: String,
    pub time: u64,
}

impl GameLog {
    pub fn new(log: String, time: u64) -> Self {
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

    pub fn switch_sides(&mut self) {
        self.batting = match self.batting {
            BattingPitching::Batting => BattingPitching::Pitching,
            BattingPitching::Pitching => BattingPitching::Batting,
        }
    }
}