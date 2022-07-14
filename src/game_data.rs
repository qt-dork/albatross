
use std::time::Duration;

use crate::util::rng::Rand32;
use crate::util::comp::*;
use crate::league::League;
use crate::util::messaging::{Message, MessageLog};
use crate::weather::Weather;

#[derive(Clone, Debug, PartialEq)]
pub struct GameData {
    pub id: Vec<GameId>,
    next_id: GameId,
    pub data: Vec<GameDatum>,
    pub day: Comp<u32>,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            id: Vec::new(),
            next_id: 0,
            data: Vec::new(),
            day: Comp::new(),
        }
    }

    pub fn create_new_game(&mut self, league: &League, day: u32) -> GameId {
        let id = self.next_id;
        self.next_id += 1;
        self.id.push(id);
        self.day.insert(id, day);
        

        id
    }

    pub fn clear(&mut self) {
        self.id.clear();
        self.data.clear();
        self.day.clear();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameDatum {
    pub id: GameId,

    pub rng: Rand32,

    pub home: TeamId,
    pub away: TeamId,

    pub home_pitcher: PlayerId,
    pub away_pitcher: PlayerId,
    pub home_bat: PlayerId,
    pub away_bat: PlayerId,

    pub day: u32,
    pub start_time: u128,
    pub current_time: u128,

    pub weather: Weather,
    // Mixed includes data for weather_name, but you can get the name from the weather enum by calling weather.as_str()

    pub inning: u32,
    pub top_inning: bool,

    // Mixed includes batting_team and pitching_team but I think there's got to be a better way.
    pub home_batter: PlayerId,
    pub away_batter: PlayerId,
    // I don't know if this is necessary, since I'll simply have a function that returns a random batter
    // defender: PlayerId,
    
    pub home_score: i32,
    pub away_score: i32,
    pub home_wins: u32,
    pub away_wins: u32,
    
    pub balls: u32,
    pub strikes: u32,
    pub outs: u32,

    pub currently_out: bool,
    pub currently_out_from_steal: bool,

    pub home_has_scored: bool,
    pub away_has_scored: bool,

    pub bases: Vec<Option<PlayerId>>,

    pub log: MessageLog,

    // Mixed also includes game_name.
}

impl GameDatum {
    pub fn new(league: &League, 
        id: GameId, 
        day: u32,
        home: TeamId,
        away: TeamId,
        start_time: u128,
    ) -> Self {
        let favors: Vec<&u32> = [home, away].iter().map(|team| {
            league.teams.favor.get(team).unwrap()
        }).collect();
        let (&home_favor, &away_favor) = (favors[0], favors[1]);
        let seed = {
            day * home_favor.pow(away_favor) + away_favor.pow(home_favor)
        };
        GameDatum {
            id,
            rng: Rand32::new(seed as u64),
            home,
            away,
            home_pitcher: 0,
            away_pitcher: 0,
            home_bat: 0,
            away_bat: 0,
            day,
            start_time,
            current_time: start_time,
            weather: Weather::default(),
            inning: 1,
            top_inning: true,
            home_batter: 0,
            away_batter: 0,
            home_score: 0,
            away_score: 0,
            home_wins: 0,
            away_wins: 0,
            balls: 0,
            strikes: 0,
            outs: 0,
            currently_out: false,
            currently_out_from_steal: false,
            home_has_scored: false,
            away_has_scored: false,
            bases: vec![None, None, None],
            log: MessageLog::new(),
        }
    }

    pub fn score(&mut self, player: PlayerId, team: TeamId) {
        if team == self.home {
            self.home_score += 1;
            self.home_has_scored = true;
        } else {
            self.away_score += 1;
            self.away_has_scored = true;
        }
        // self.log.log(Message::Scores(player), time, is_special)
    }

    /// Logs a message to the game log. This is a wrapper around the [`MessageLog::log`] function.
    /// 
    /// All log functions except [`GameDatum::log_with_time`] assume that you're logging a message at the current time.
    pub fn log(&mut self, message: Message) {
        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        self.log.log(message, time, false);
    }

    pub fn log_with_time(&mut self, message: Message, time: Duration) {
        self.log.log(message, time, false);
    }

    pub fn log_special(&mut self, message: Message) {
        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        self.log.log(message, time, true);
    }
}
