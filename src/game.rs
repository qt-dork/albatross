use std::thread::{sleep, self};
use std::time::{Duration, SystemTime};

use crate::messaging::{MessageLog, Message};
// The point of this file is to generate most of the game logic so it can be easily called via a functional interface.
use crate::team::Team;
use crate::player::Player;
use crate::java_random::Random;

const TICK: u128 = 0;

pub struct Game {
    pub rng: Random,

    pub home: Team,
    pub away: Team,

    pub home_pitcher: Player,
    pub away_pitcher: Player,

    pub day: usize,
    pub start_time: u128, // NOTE: I feel like this could be collapsed into a single field? Like a start_time current_time struct.
    pub current_time: u128,
    pub message_log: MessageLog,
    // pub weather: Weather, // TODO: implement weather

    pub inning: u32,
    pub top: bool,

    pub batter: Player,
    pub defender: Player,

    pub scores: (f64, f64),
    pub wins: (i32, i32),
    pub balls: i32,
    pub strikes: i32,
    pub outs: i32,
    pub home_bat: usize, // Maybe this should be EitherOr?
    pub away_bat: usize, // Wait no it can't. It's a u32.

    pub bases: Vec<Option<Player>>,
}

impl Game {
    pub fn new(home: Team, away: Team, day: usize, start_time: u128) -> Self {
        let seed: i64 = (day + 4 + home.get_favor() as usize + away.get_favor() as usize) as i64; // Changes in update. TODO: fix this

        let mut rng = Random::new(seed);
        Game {
            rng,

            home,
            away,

            // Placeholders
            home_pitcher: Player::default(&mut rng),
            away_pitcher: Player::default(&mut rng),
        
            day,
            start_time,
            current_time: 0,
            message_log: MessageLog::new(),
            // weather: , // TODO: implement weather
        
            inning: 1,
            top: true,
        
            batter: Player::default(&mut rng), // Placeholder
            defender: Player::default(&mut rng), // Placeholder
        
            scores: (0.0, 0.0),
            wins: (0, 0),
            balls: 0,
            strikes: 0,
            outs: 0,
            home_bat: 0,
            away_bat: 0,
        
            bases: Vec::from([None, None, None]),
        }
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.into_iter()
    }
    pub fn teams_mut(&mut self) -> impl Iterator<Item = &mut Team> {
        [&mut self.away, &mut self.home].into_iter()
    }
    pub fn teams_batting(&self) -> &Team {
        if self.top {
            &self.away
        } else {
            &self.home
        }
    }
    pub fn teams_batting_mut(&mut self) -> &mut Team {
        if self.top {
            &mut self.away
        } else {
            &mut self.home
        }
    }
    pub fn teams_pitching(&self) -> &Team {
        if self.top {
            &self.home
        } else {
            &self.away
        }
    }
    pub fn teams_pitching_mut(&mut self) -> &mut Team {
        if self.top {
            &mut self.home
        } else {
            &mut self.away
        }
    }
    pub fn pitchers(&self) -> impl Iterator<Item = &Player> {
        [&self.away_pitcher, &self.home_pitcher].into_iter()
    }
    pub fn pitchers_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        [&mut self.away_pitcher, &mut self.home_pitcher].into_iter()
    }
    pub fn pitchers_batting(&self) -> &Player {
        if self.top {
            &self.away_pitcher
        } else {
            &self.home_pitcher
        }
    }
    pub fn pitchers_batting_mut(&mut self) -> &mut Player {
        if self.top {
            &mut self.away_pitcher
        } else {
            &mut self.home_pitcher
        }
    }
    pub fn pitchers_pitching(&self) -> &Player {
        if self.top {
            &self.home_pitcher
        } else {
            &self.away_pitcher
        }
    }
    pub fn pitchers_pitching_mut(&mut self) -> &mut Player {
        if self.top {
            &mut self.home_pitcher
        } else {
            &mut self.away_pitcher
        }
    }
    pub fn bat(&self) -> impl Iterator<Item = &usize> {
        [&self.away_bat, &self.home_bat].into_iter()
    }
    pub fn bat_batting(&self) -> &usize {
        if self.top {
            &self.away_bat
        } else {
            &self.home_bat
        }
    }
    pub fn bat_batting_mut(&mut self) -> &mut usize {
        if self.top {
            &mut self.away_bat
        } else {
            &mut self.home_bat
        }
    }
    pub fn bat_pitching(&self) -> &usize {
        if self.top {
            &self.home_bat
        } else {
            &self.away_bat
        }
    }
    pub fn bat_pitching_mut(&mut self) -> &mut usize {
        if self.top {
            &mut self.home_bat
        } else {
            &mut self.away_bat
        }
    }

    // Figure out a better way to do this
    pub fn start_game(&mut self) {
        let len_home = &self.home.get_active_pitchers().len();
        let pitcher_home = &self.home
            .get_active_pitchers()[(self.day - 1 + len_home) % len_home];
        let len_away = &self.away.get_active_pitchers().len();
        let pitcher_away = &self.away
            .get_active_pitchers()[(self.day - 1 + len_away) % len_away].clone();
        
        self.home_pitcher = pitcher_home.clone();
        self.away_pitcher = pitcher_away.clone();
    }

    // Placeholder
    pub fn simulate_game(&mut self) {
        self.start_game();

        self.update(Message::StartGame, TICK);

        let sys_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        
        let cur_time = sys_time.as_secs();

        while !self.is_game_over() {
            self.do_inning();
        }

        self.update(Message::EndGameScore(self.home.get_name(), self.scores.0, self.away.get_name(), self.scores.1), TICK);
        self.update(Message::GameOver, TICK);
        self.give_wins();
    }

    // Placeholder
    pub fn do_inning(&mut self) {
        self.update(Message::InningStart(self.top, self.inning, self.teams_batting().get_name(), self.teams_pitching().get_name()), TICK);

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
            self.update(Message::Out(self.outs), 0);
        }

        // Do end of inning stuff
        self.outs = 0;
        match self.top {
            true => self.top = false,
            false => {
                self.top = true;
                self.update(Message::InningToOuting(self.inning), TICK);
                self.print_score(); // NOTE: I don't like this

                self.inning += 1;
            },
        }
    }

    // NOTE: This feels off idk. Figure out a better way to do this
    pub fn is_game_over(&self) -> bool {
        self.scores.0 != self.scores.1 && self.inning > 9 && self.top == true
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

    fn update(&mut self, log: Message, time: u128) {
        // Unused since Duration can sleep for x amount of time (which is a cursed solution but I'll find a better one later)
        let current_time = self.current_time; // See if there's a better way to handle current time?
        self.message_log.log(log, time);
        self.current_time += time;
    }

    // Okay here's the worst function in the code. I'm sorry.
    fn do_pitch(&mut self) {
        let cur_pitcher = self.pitchers_pitching().clone();
        let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers_pitching().clone().pinpointedness.value();
        let bat_value = self.rng.next_f64() * 10.0 + self.batter.density.value();

        if bat_value <= pitch_value {
            self.ball();
        } else {
            let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers_pitching().fun.value();
            let bat_value = self.rng.next_f64() * 10.0 + self.batter.number_of_eyes.value();
            if bat_value <= pitch_value {
                self.strikes += 1;
                if self.has_struck_out() {
                    self.update(Message::StruckOutLooking(self.batter.get_name(), (self.balls, self.strikes)), TICK);
                } else {
                    self.update(Message::StrikeLooking((self.balls, self.strikes)), TICK);
                }
            } else {
                let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers_pitching().dimensions.value();
                let bat_value = self.rng.next_f64() * 10.0 + self.batter.malleability.value();
                if bat_value <= pitch_value {
                    self.strikes += 1;
                    if self.has_struck_out() {
                        self.update(Message::StruckOutSwinging(self.batter.get_name(), (self.balls, self.strikes)), TICK);
                    } else {
                        self.update(Message::StrikeSwinging((self.balls, self.strikes)), TICK);
                    }
                }
            }
            if !self.has_struck_out() {
                let pitch_value = self.rng.next_f64() * 10.0 + self.pitchers_pitching().powder.value();
                let bat_value = self.rng.next_f64() * 10.0 + self.batter.splash.value();
                if bat_value <= pitch_value {
                    self.strikes += 1;
                    while self.has_struck_out() {
                        self.strikes -= 1;
                    }
                    self.update(Message::FoulBall((self.balls, self.strikes)), TICK);
                }
            } else {
                let defender = self.get_random_defender();
                let bat_value = self.rng.next_f64() * 10.0 + self.batter.aggression.value();
                let defense_value = self.rng.next_f64() * 10.0 + defender.mathematics.value();
                if bat_value <= defense_value {
                    // What the fuck?
                    self.strikes += 100;
                    self.update(Message::Flyout(self.batter.get_name(), self.defender.get_name()),TICK)
                } else {
                    let bat_value = self.rng.next_f64() * 10.0 + self.batter.hit_points.value();
                    let defense_value = self.rng.next_f64() * 10.0 + defender.damage.value();
                    if bat_value <= defense_value {
                        self.strikes += 100;
                        self.update(Message::Groundout(self.batter.get_name(), self.defender.get_name()), TICK);
                    } else {
                        let mut bases_run = 0;
                        loop {
                            let bat_value = self.rng.next_f64() * 10.0 + self.batter.effort.value();
                            let defense_value = self.rng.next_f64() * 10.0 + defender.carcinization.value();
                            bases_run += 1;
                            if bat_value <= defense_value {
                                break;
                            }
                        }
                        self.update(Message::Hit(self.batter.get_name(), bases_run), TICK);
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
        let defender = self.get_random_defender();

        // You should not be accessing these values inside
        // of the function imo. They should be passed in.
        let urge = self.rng.next_f64() * 10.0 + self.bases[base_num as usize].as_ref().unwrap().arrogance.value() - defender.rejection.value(); // I know using unwrap is prolly a bad idea here but this entire code is a bad idea
        if urge < 9.9 {
            return;
        }

        let steal_value = self.rng.next_f64() * 10.0 + self.bases[base_num as usize].clone().unwrap().dexterity.value();
        let defense_value = self.rng.next_f64() * 10.0 + defender.wisdom.value();

        if steal_value > defense_value {
            // The last part of the message will be
            // by the position of the base stolen
            self.update(Message::Steal(self.bases[base_num as usize].clone().unwrap().get_name(), base_num), TICK);

            // Holy shit why is this passing in an entire
            // Player to a score function
            self.score(&self.bases[base_num as usize].clone().unwrap());
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
            self.update(Message::CaughtStealing(self.bases[base_num].clone().unwrap().get_name(), base_num), TICK);

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
        if self.top {
            self.scores.0 += 1.0;
        } else {
            self.scores.1 += 1.0;
        }
        self.update(Message::Scores(p.get_name()), 0);
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
            self.home.get_name(), // TODO: Make this the current weather once that is implemented
            self.away.get_name(),
            self.home.get_name(), 
            self.day
        )
    }

    // Precondition: simulate_game() has been called already
    pub fn is_live(&self) -> bool {
        self.current_time >= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() && self.start_time <= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
    }

    pub fn get_random_defender(&mut self) -> Player {
        let players = self.teams_pitching().get_active_batters();
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

    // Idk how to add this to the game log :/
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
            self.update(Message::Walk(self.batter.get_name()), TICK);
            self.walk();
            self.set_next_batter();
        } else {
            self.update(Message::Ball((self.balls, self.strikes)), TICK)
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
        self.update(Message::CurrentScore(self.home.abbreviation.clone(), self.scores.0, self.scores.1, self.away.abbreviation.clone()), 0);
    }

    /// Retrieves the teams from the Game struct. The use
    /// is for updating the real Teams which is stored
    /// outside of the Game.
    pub fn get_teams(&self) -> (Team, Team) {
        (self.home.clone(), self.away.clone())
    }

    // TODO: This is a void function and needs to be made not bad later
    fn give_wins(&mut self) {
        if self.scores.0 > self.scores.1 {
            self.wins.0 += 1;
            if self.day < 100 {
                &mut self.home.add_win();
                &mut self.away.add_loss();
            }
        } else {
            self.wins.1 += 1;
            if self.day < 100 {
                &mut self.away.add_win();
                &mut self.home.add_loss();
            }
        }
        if self.day < 100 {
            &mut self.home.add_win_by(self.wins.0);
            &mut self.away.add_win_by(self.wins.1);
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
        
        *self.bat_batting_mut()+= 1;
        let batting_team = self.teams_batting().get_active_batters();
        self.batter = batting_team[(self.bat_batting() - 1) % batting_team.len()].clone();

        self.update(Message::NextBatter(self.batter.get_name(), self.teams_batting().get_name()), TICK);
    }

    pub fn play_logs(&mut self) {
        for i in 0..self.message_log.len().unwrap() {
            let log = self.message_log.pop_front().unwrap();
            while SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() < log.1 {
                thread::sleep(Duration::from_millis(TICK.try_into().unwrap()));
            }
            println!("{}", log.0.message_line());
        }
    }
}

impl<'a> IntoIterator for &'a Game {
    type Item = &'a Team;
    type IntoIter = std::array::IntoIter<&'a Team, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.away, &self.home].into_iter()
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