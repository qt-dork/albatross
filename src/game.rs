use std::thread::sleep;
use std::time::{Duration, SystemTime};

// The point of this file is to generate most of the game logic so it can be easily called via a functional interface.
use crate::team::Team;
use crate::player::Player;
use crate::java_random::Random;

const TICK: u128 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopOfInning {
    Top,
    Bottom,
}

pub struct Game {
    pub rng: Random,

    pub teams: EitherOr<Team>,
    pub pitchers: EitherOr<Player>,

    pub day: usize,
    pub start_time: u128, // NOTE: I feel like this could be collapsed into a single field? Like a start_time current_time struct.
    pub current_time: u128,
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
    pub active_team_bat: usize, // Maybe this should be EitherOr?
    pub inactive_team_bat: usize, // Wait no it can't. It's a u32.

    pub bases: Vec<Option<Player>>,
}

impl Game {
    pub fn new(home: Team, away: Team, day: usize, start_time: u128) -> Self {
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
        
            bases: Vec::from([None, None, None]),
        }
    }

    // Figure out a better way to do this
    pub fn start_game(&mut self) {
        let len_home = self.teams.home().get_active_pitchers().len();
        let pitcher_home = &self.teams.home()
            .get_active_pitchers()[(self.day - 1 + len_home) % len_home];
        let len_away = self.teams.away().get_active_pitchers().len();
        let pitcher_away = self.teams.away()
            .get_active_pitchers()[(self.day - 1 + len_away) % len_away].clone();
        
        self.pitchers = EitherOr::new(pitcher_home.clone(), pitcher_away);
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
    pub fn do_inning(&mut self) {
        let mut message = String::new();
        if self.top == TopOfInning::Top {
            message.push_str("Top");
        } else {
            message.push_str("Bottom");
        }
        message += &format!(" of {}, {} batting. {} pitching.", self.inning, self.teams.batting().get_name(), self.teams.pitching().get_name());
        self.update(message.clone(), TICK);

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
                self.update(format!("Inning {} is now an Outing.", self.inning), TICK);
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

    fn update(&mut self, log: String, time: u128) {
        // Unused since Duration can sleep for x amount of time (which is a cursed solution but I'll find a better one later)
        // let current_time = self.current_time; // See if there's a better way to handle current time?
        self.game_log.push(GameLog {
            log,
            time,
        });
        self.current_time += time;
    }

    // Okay here's the worst function in the code. I'm sorry.
    fn do_pitch(&mut self) {
        let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers.pitching().pinpointedness;
        let bat_value = self.rng.next_f64() * 10.0 + self.batter.density;

        if bat_value <= pitch_value {
            self.ball();
        } else {
            let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers.pitching().fun;
            let bat_value = self.rng.next_f64() * 10.0 + self.batter.number_of_eyes;
            if bat_value <= pitch_value {
                self.strikes += 1;
                if self.has_struck_out() {
                    let mut message = format!("{} struck out! ", self.batter.name);
                    message.push_str(self.format_balls_strikes().as_str());
                    self.update(message, TICK);
                } else {
                    let mut message = "Strike, looking. ".to_string();
                    message.push_str(self.format_balls_strikes().as_str());
                    self.update(message, TICK);
                }
            } else {
                let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers.pitching().dimensions;
                let bat_value = self.rng.next_f64() * 10.0 + self.batter.malleability;
                if bat_value <= pitch_value {
                    self.strikes += 1;
                    if self.has_struck_out() {
                        let mut message = format!("{} strikes out swinging. ", self.batter.name);
                        message.push_str(self.format_balls_strikes().as_str());
                        self.update(message, TICK);
                    } else {
                        let mut message = "Strike, swinging. ".to_string();
                        message.push_str(self.format_balls_strikes().as_str());
                        self.update(message, TICK);
                    }
                }
            }
            if !self.has_struck_out() {
                let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers.pitching().powder;
                let bat_value = self.rng.next_f64() * 10.0 + self.batter.splash;
                if bat_value <= pitch_value {
                    self.strikes += 1;
                    while self.has_struck_out() {
                        self.strikes -= 1;
                    }
                    let mut message = "Foul ball. ".to_string();
                    message.push_str(self.format_balls_strikes().as_str());
                    self.update(message, TICK);
                }
            } else {
                let defender = self.get_random_defender();
                let bat_value = self.rng.next_f64() * 10.0 + self.batter.aggression;
                let defense_value = self.rng.next_f64() * 10.0 + defender.mathematics;
                if bat_value <= defense_value {
                    // What the fuck?
                    self.strikes += 100;
                    self.update(
                        format!(
                            "{} hit a flyout to {}.",
                            self.batter.get_name(),
                            self.defender.get_name()
                        ),
                        TICK
                    )
                } else {
                    let bat_value = self.rng.next_f64() * 10.0 + self.batter.hit_points;
                    let defense_value = self.rng.next_f64() * 10.0 + defender.damage;
                    if bat_value <= defense_value {
                        self.strikes += 100;
                        self.update(
                            format!(
                                "{} hit a ground out to {}.",
                                self.batter.get_name(),
                                self.defender.get_name()
                            ),
                            TICK
                        )
                    } else {
                        let mut bases_run = 0;
                        loop {
                            let bat_value = self.rng.next_f64() * 10.0 + self.batter.effort;
                            let defense_value = self.rng.next_f64() * 10.0 + defender.carcinization;
                            bases_run += 1;
                            if bat_value <= defense_value {
                                break;
                            }
                        }
                        let mut message = format!(
                            "{} hits a ",
                            self.batter.get_name()
                        );
                        match bases_run {
                            1 => {
                                message.push_str("Single!");
                            },
                            2 => {
                                message.push_str("Double!");
                            },
                            3 => {
                                message.push_str("Triple!");
                            },
                            _ => {
                                message.push_str("Home run!");
                            },
                        }
                        self.update(message, TICK);
                        self.advance_baserunners(bases_run);
                        self.set_next_batter();
                    }
                }
            }
        }
    }

    // God this function sucks so much shit I need to break
    // this up into separate rolls for the sake of my sanity
    fn steal_attempt(&mut self, base_num: usize) {
        // Defender should be set after the urge check so
        // you're not doing extra unnecessary work
        let defender = self.get_random_defender();

        // You should not be accessing these values inside
        // of the function imo. They should be passed in.
        let urge = self.rng.next_f64() * 10.0 + self.bases[base_num as usize].as_ref().unwrap().arrogance - defender.rejection; // I know using unwrap is prolly a bad idea here but this entire code is a bad idea
        if urge < 9.9 {
            return;
        }

        let steal_value = self.rng.next_f64() * 10.0 + self.bases[base_num as usize].clone().unwrap().dexterity;
        let defense_value = self.rng.next_f64() * 10.0 + defender.wisdom;

        if steal_value > defense_value {
            // The last part of the message will be
            // by the position of the base stolen
            let mut message = format!("{} steals ", self.bases[base_num as usize].clone().unwrap().get_name());
            match base_num {
                0 => message.push_str("second base!"),
                1 => message.push_str("third base!"),
                2 => message.push_str("home!"),

                // Fourth base isn't out yet
                _ => panic!("Invalid base number"),
            }
            self.update(message, TICK);

            // Holy shit why is this passing in an entire
            // Player to a score function
            self.score(&self.bases[base_num].clone().unwrap());
            // Why does this account for variable bases but
            // other parts of your code doesn't?
            if base_num <= self.bases.len() - 1 {
                // This feels bad for some reason
                self.bases[base_num + 1] = self.bases[base_num].clone();
            }
            // Fun fact: The original code sets this to null
            self.bases[base_num] = None;

        // It occurred to me just now that this is all a 
        // nested if statement.
        } else {
            let mut message = format!("{} gets caught stealing ", self.bases[base_num].clone().unwrap().get_name());
            match base_num {
                0 => message.push_str("second base."),
                1 => message.push_str("third base."),
                2 => message.push_str("home."),

                // Fourth base isn't out yet
                _ => panic!("Invalid base number"),
            }
            self.update(message, TICK);

            // What the fuck? Why is this 100?
            self.strikes += 100;
            self.bases[base_num] = None;
        }
        self.do_steals();
    }

    // Once again a function that just reaches in and grabs
    // data from the struct and mutates it for game shit.
    // Also a void function.
    //
    // Also the name for this function sucks.
    fn do_steals(&mut self) {
        for (i, base) in self.bases.clone().iter_mut().enumerate() {
            if let Some(player) = base {
                if (i == self.bases.len() - 1 || self.bases[i + 1].is_none()) && !self.has_struck_out() {
                    // Wait so this function passes in the
                    // base number of the player that's
                    // stealing instead of the player itself?
                    self.steal_attempt(i);
                }
            }
        }
    }

    // Incredibly cursed function.
    fn advance_baserunners(&mut self, base_num: usize) {
        let mut scored = false;
        let mut new_bases: Vec<Option<Player>> = self.bases.clone();
        let mut score_queue: Vec<Player> = Vec::new();
        for (i, base) in self.bases.iter_mut().enumerate().rev() {
            if let Some(player) = base {
                if i + base_num >= new_bases.len() {
                    // self.score(player); // Breaks borrow checker
                    score_queue.push(player.clone());
                    new_bases[i] = None;
                    scored = true;
                } else {
                    new_bases[i + base_num] = new_bases[i].clone();
                    new_bases[i] = None;
                }
            }
        }
        self.bases = new_bases;
        if scored {
            for player in score_queue {
                self.score(&player);
            }
        }
        if scored {
            self.print_score();
        }
    }

    fn score(&mut self, p: &Player) {
        if self.top == TopOfInning::Top {
            self.scores.0 += 1.0;
        } else {
            self.scores.1 += 1.0;
        }
        self.update(format!("{} scores!", p.get_name()), 0);
    }

    // Unused code
    // pub fn wants_to_steal(&mut self, p: &Player) -> bool {
    //     let defender = self.get_random_defender();
    //     let urge = self.rng.next_f64() * 10.0 + p.arrogance - defender.rejection;

    //     // This is different from the other function
    //     urge >= 9.5
    // }

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
        self.current_time >= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() && self.start_time <= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
    }

    pub fn get_random_defender(&mut self) -> Player {
        let players = self.teams.pitching().get_active_batters();
        let random_number = (self.rng.next_f64() * players.len() as f64) as usize;

        players[random_number].clone()
    }

    /// Returns true if the batter has struck out.
    /// 
    /// NOTE: This will need to be updated to be variable
    /// in the future if features like fourth strike are
    /// added
    pub fn has_struck_out(&self) -> bool {
        self.strikes >= 3
    }

    pub fn format_balls_strikes(&self) -> String {
        let mut b = self.balls.to_string();
        let mut s = self.strikes.to_string();

        if self.balls < 0 {
            b = format!("(-{})", self.balls.abs());
        }
        if self.strikes < 0 {
            s = format!("(-{})", self.strikes.abs());
        }
        return format!("{}-{}", b, s);
    }

    pub fn clear_bases(&mut self) {
        self.bases = vec![None, None, None];
    }

    pub fn ball(&mut self) {
        self.balls += 1;
        if self.balls >= 4 {
            // For some reason in the original code, the
            // Player class has a get_name() function but
            // it's never used. Mixed just grabs the name
            // from the variables.
            self.update(format!("{} draws a walk.", self.batter.get_name()), TICK);
            self.walk();
            self.set_next_batter();
        } else {
            self.update(format!("Ball. {} - {}", self.balls, self.strikes), TICK)
        }
    }

    pub fn walk(&mut self) {
        let mut scored = false;
        // This feels like the borrow checker will hate it
        let mut moving = self.batter.clone();
        let mut new_bases = self.bases.clone();
        let mut score_queue: Vec<Player> = Vec::new();
        for (i, base) in self.bases.iter_mut().enumerate() {
            if let Some(player) = base {
                // Swap the moving player with the player on base
                new_bases[i] = Some(moving.clone());
                moving = player.clone();
                if i == new_bases.len() - 1 {
                    // self.score(&moving); // Breaks borrow checker
                    score_queue.push(moving.clone());
                    scored = true;
                }
            } else {
                self.bases[i] = Some(moving);
                break;
            }
        }
        if scored {
            for player in score_queue {
                self.score(&player);
            }
            self.print_score();
        }
    }

    fn print_score(&mut self) {
        let message = format!("[Current score is {} {}-{} {}]", self.teams.home().abbreviation.clone(), score_as_string(self.scores.0), score_as_string(self.scores.1), self.teams.away().abbreviation.clone());
        self.update(message, 0);
    }

    /// Retrieves the teams from the Game struct. The use
    /// is for updating the real Teams which is stored
    /// outside of the Game.
    pub fn get_teams(&self) -> (Team, Team) {
        (self.teams.home().clone(), self.teams.away().clone())
    }

    // TODO: This is a void function and needs to be made not bad later
    fn give_wins(&mut self) {
        if self.scores.0 > self.scores.1 {
            self.wins.0 += 1;
            if self.day < 100 {
                self.teams.mut_home().add_win();
                self.teams.mut_away().add_loss();
            }
        } else {
            self.wins.1 += 1;
            if self.day < 100 {
                self.teams.mut_away().add_win();
                self.teams.mut_home().add_loss();
            }
        }
        if self.day < 100 {
            self.teams.mut_home().add_win_by(self.wins.0);
            self.teams.mut_away().add_win_by(self.wins.1);
        }
    }

    // Another void function aaaaaaa
    // Sets the next batter unless the inning is about to end
    fn set_next_batter(&mut self) {
        if self.is_inning_over() {
            return;
        }
        self.strikes = 0;
        self.balls = 0;
        
        self.active_team_bat+= 1;
        let batting_team = self.teams.batting().get_active_batters();
        self.batter = batting_team[(self.active_team_bat - 1) % batting_team.len()].clone();

        self.update(format!("{} batting for the {}.", self.batter.get_name(), self.teams.batting().get_name()), TICK);
    }

    pub fn play_logs(&self) {
        for log in self.game_log.iter() {
            sleep(Duration::from_millis(log.time.try_into().unwrap()));
            println!("{}", log.log);
        }
    }
}


// Game function but not part of game struct
pub fn score_as_string(score: f64) -> String {
    let mut message = String::new();
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
    pub time: u128,
}

impl GameLog {
    pub fn new(log: String, time: u128) -> Self {
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

    pub fn batting(&self) -> &T {
        match self.batting {
            BattingPitching::Batting => &self.items.0,
            BattingPitching::Pitching => &self.items.1,
        }
    }

    pub fn mut_batting(&mut self) -> &mut T {
        match self.batting {
            BattingPitching::Batting => &mut self.items.0,
            BattingPitching::Pitching => &mut self.items.1,
        }
    }

    pub fn set_batting(&mut self, item: T) {
        match self.batting {
            BattingPitching::Batting => self.items.0 = item,
            BattingPitching::Pitching => self.items.1 = item,
        }
    }

    pub fn pitching(&self) -> &T {
        match self.batting {
            BattingPitching::Batting => &self.items.1,
            BattingPitching::Pitching => &self.items.0,
        }
    }

    pub fn mut_pitching(&mut self) -> &mut T {
        match self.batting {
            BattingPitching::Batting => &mut self.items.1,
            BattingPitching::Pitching => &mut self.items.0,
        }
    }

    pub fn set_pitching(&mut self, item: T) {
        match self.batting {
            BattingPitching::Batting => self.items.1 = item,
            BattingPitching::Pitching => self.items.0 = item,
        }
    }

    pub fn home(&self) -> &T {
        &self.items.0
    }

    pub fn mut_home(&mut self) -> &mut T {
        &mut self.items.0
    }

    pub fn set_home(&mut self, item: T) {
        self.items.0 = item;
    }

    pub fn away(&self) -> &T {
        &self.items.1
    }

    pub fn mut_away(&mut self) -> &mut T {
        &mut self.items.1
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