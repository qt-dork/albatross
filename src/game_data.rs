
use crate::java_random::Random;
use crate::comp::{Comp, CompIter, EntityId, TeamId, PlayerId, GameId};
use crate::weather::Weather;

#[derive(Clone, Debug, PartialEq)]
pub struct GameData {
    pub id: Vec<GameId>,
    pub data: Vec<GameDatum>,
    pub day: Comp<u32>,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            id: Vec::new(),
            data: Vec::new(),
            day: Comp::new(),
        }
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

    rng: Random,

    home: TeamId,
    away: TeamId,

    home_pitcher: PlayerId,
    away_pitcher: PlayerId,
    home_bat: PlayerId,
    away_bat: PlayerId,

    pub day: u32,
    start_time: u128,
    current_time: u128,

    weather: Weather,
    // Mixed includes data for weather_name, but you can get the name from the weather enum by calling weather.as_str()

    inning: u32,
    top_inning: bool,

    // Mixed includes batting_team and pitching_team but I think there's got to be a better way.
    home_batter: PlayerId,
    away_batter: PlayerId,
    // I don't know if this is necessary, since I'll simply have a function that returns a random batter
    // defender: Comp<TeamId>,
    
    home_score: u32,
    away_score: u32,
    home_wins: u32,
    away_wins: u32,
    
    balls: u32,
    strikes: u32,
    outs: u32,

    currently_out: bool,
    currently_out_from_steal: bool,

    home_has_scored: bool,
    away_has_scored: bool,

    bases: Bases<PlayerId>,

    // Mixed also includes game_name.
}

