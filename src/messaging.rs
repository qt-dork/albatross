/// This handles all the messaging between the game and the user.
/// This is directly inspired by RustRoguelike's messaging system, which you can find here:
/// https://github.com/nsmryan/RustRoguelike/blob/master/roguelike_core/src/messaging.rs

use std::{collections::VecDeque, time::Duration};

use crate::{comp::{TeamId, PlayerId}, league::League};

pub type BallsStrikes = (i32, i32);

// Also derives Copy, Deserialize, and Serialize
// Copy doesn't work here because we store player names instead of ids to players, whose names are retrieved from the database.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    // Game event messages
    /// Team 1, Team 2
    AnnounceMatchup(TeamId, TeamId),
    StartGame,
    /// Top, Inning, Team1, Team 2
    InningStart(bool, u32, TeamId, TeamId),
    /// Team1Abbr, Team1Score, Team2Score, Team2Abbr
    CurrentScore(TeamId, f64, f64, TeamId),
    /// Stealer Name, Base Stolen
    Steal(PlayerId, usize),
    /// Stealer Name, Base Stolen
    CaughtStealing(PlayerId, usize),
    /// Batter Name
    Walk(PlayerId), 
    /// BallsStrikes
    Ball(BallsStrikes),
    /// Batter Name, Balls Strikes
    StruckOutLooking(PlayerId, BallsStrikes),
    /// Balls Strikes
    StrikeLooking(BallsStrikes),
    /// Batter Name, Balls Strikes
    StruckOutSwinging(PlayerId, BallsStrikes),
    /// Balls Strikes
    StrikeSwinging(BallsStrikes), 
    /// Balls Strikes
    FoulBall(BallsStrikes), 
    /// Batter Name, Defender Name
    Flyout(PlayerId, PlayerId), 
    /// Batter Name, Defender Name
    Groundout(PlayerId, PlayerId), 
    // NOTE: Maybe I should have Bases Hit be its own enum?
    /// Batter Name
    Scores(PlayerId), 
    /// Batter Name, Bases Hit
    Hit(PlayerId, usize), 
    // Figure something out for the format_balls_strikes() function
    /// Batter Name, Team Name
    NextBatter(PlayerId, TeamId), 
    /// Number of outs
    Out(i32),
    /// Inning
    InningToOuting(u32),
    /// Team1Name, Team1Score, Team2Name, Team2Score
    EndGameScore(TeamId, f64, TeamId, f64), 
    GameOver,

    // Weather specific messages
}

impl Message {
    pub fn message_line(&self, league: &League) -> String {
        match self {
            Message::AnnounceMatchup(team1, team2) => {
                format!("{:?} vs. {:?}",
                    league.teams.name[team1].clone(),
                    league.teams.name[team2].clone()
                )
            },
            Message::StartGame => {
                "Blay pall!".to_string()
            },
            Message::InningStart(top, inning, team1, team2) => {
                match top {
                    true => {
                        format!("Top of {}, {:?} batting. {:?} pitching.", 
                            inning,
                            league.teams.name[team1].clone(),
                            league.teams.name[team2].clone()
                        )
                    },
                    false => {
                        format!("Bottom of {}, {:?} batting. {:?} pitching.",
                            inning,
                            league.teams.name[team1].clone(),
                            league.teams.name[team2].clone()
                        )
                    }
                }
            },
            Message::CurrentScore(team1, score1, score2, team2) => {
                format!("[Current score is {:?} {:?}-{:?} {:?}]", 
                    league.teams.abbreviation[team1].clone(),
                    Self::score_as_string(score1),
                    Self::score_as_string(score2),
                    league.teams.abbreviation[team2].clone()
                )
            },
            Message::Steal(stealer, base) => {
                let stolen_base = match base {
                    0 => {
                        "second base"
                    },
                    1 => {
                        "third base"
                    },
                    2 => {
                        "home"
                    },
                    _ => {
                        // Fourth base isn't out yet
                        panic!("Invalid base number: {}", base);
                    }
                };
                format!("{:?} steals {:?}!",
                    league.players.name[stealer].clone(),
                    stolen_base
                )
            },
            Message::CaughtStealing(stealer, base) => {
                let stolen_base =match base {
                    0 => {
                        "second base"
                    },
                    1 => {
                        "third base"
                    },
                    2 => {
                        "home"
                    },
                    _ => {
                        // Fourth base isn't out yet
                        panic!("Invalid base number: {}", base);
                    }
                };
                format!("{:?} gets caught stealing {:?}.",
                    league.players.name[stealer].clone(),
                    stolen_base
                )
            },
            Message::Walk(batter) => {
                format!("{:?} draws a walk.", league.players.name[batter].clone())
            },
            Message::Ball((balls, strikes)) => {
                format!("Ball. {}-{}", balls, strikes)
            },
            Message::StruckOutLooking(batter, balls_strikes) => {
                format!("{:?} strikes out looking. ",
                    league.players.name[batter].clone()
                ) + &Self::format_balls_strikes(balls_strikes)
            },
            Message::StrikeLooking(balls_strikes) => {
                "Strike, looking. ".to_owned() + &Self::format_balls_strikes(balls_strikes)
            },
            Message::StruckOutSwinging(batter, balls_strikes) => {
                format!("{:?} strikes out swinging. ", league.players.name[batter].clone()) + &Self::format_balls_strikes(balls_strikes)
            },
            Message::StrikeSwinging(balls_strikes) => {
                "Strike, swinging. ".to_owned() +  &Self::format_balls_strikes(balls_strikes)
            },
            Message::FoulBall(balls_strikes) => {
                "Foul ball. ".to_owned() + &Self::format_balls_strikes(balls_strikes)
            },
            Message::Flyout(batter, defender) => {
                format!("{:?} hit a flyout to {:?}.", 
                    league.players.name[batter].clone(),
                    league.players.name[defender].clone()
                )
            },
            Message::Groundout(batter, defender) => {
                format!("{:?} hit a ground out to {:?}!", 
                    league.players.name[batter].clone(),
                    league.players.name[defender].clone()
                )
            },
            Message::Scores(batter) => {
                format!("{:?} scores!", league.players.name[batter].clone())
            },
            Message::Hit(batter, bases_hit) => {
                // let mut message = format!("{:?} his a ", batter);
                let base = match bases_hit {
                    1 => {
                        "Single"
                    },
                    2 => {
                        "Double"
                    },
                    3 => {
                        "Triple"
                    },
                    // Eventually this might need to support the fourth base, if that's ever added
                    _ => {
                        "Home Run"
                    }
                };
                format!("{:?} hits a {:?}!",
                    league.players.name[batter].clone(),
                    base
                )
            },
            Message::NextBatter(batter, team) => {
                format!("{:?} batting for the {:?}.", 
                    league.players.name[batter].clone(),
                    league.teams.name[team].clone()
                )
            },
            Message::Out(outs) => {
                format!("[Out {}]", outs)
            },
            Message::InningToOuting(inning) => {
                format!("Inning {} is now an Outing.", inning)
            },
            //DoneFix
            Message::EndGameScore(team1, wins1, team2, wins2) => {
                format!("{:?} {:?}, {:?} {:?}",
                    league.teams.name[team1],
                    Self::score_as_string(wins1), 
                    Self::score_as_string(wins2),
                    league.teams.name[team2]
                )
            },

            Message::GameOver => {
                "\nGame over.".to_string()
            }
        }
    }

    fn format_balls_strikes((balls, strikes): &BallsStrikes) -> String {
        let b: String;
        let s: String;
        if balls < &0 {
            b = format!("({})", balls.abs());
        } else {
            b = format!("{}", balls);
        }
        if strikes < &0 {
            s = format!("({})", strikes.abs());
        } else {
            s = format!("{}", strikes);
        }
        format!("{:?}-{:?}", b, s)
    }

    fn score_as_string(score: &f64) -> String {
        let mut message = String::new();
        if score % 1.0 == 0.0 {
            message.push_str(&format!("{}", score));
        } else {
            message.push_str(&format!("{:.1}", score));
        }
        if score < &0.0 {
            message = format!("({})", message);
        }
        message
    }
}

// Also includes Serialize and Deserialize in the original
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MessageLog {
    pub messages: VecDeque<Message>,
    // I used to use a u128, but I felt that a Duration made more sense, however, please feel free to correct me if I'm wrong.
    pub time: VecDeque<Duration>,
    pub is_special: VecDeque<bool>,
}

impl MessageLog {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> Option<usize> {
        if self.messages.len() != self.time.len() && self.messages.len() != self.is_special.len() {
            None
        } else {
            Some(self.messages.len())
        }
    }

    pub fn pop_front(&mut self) -> Option<(Message, Duration)> {
        self.messages.pop_front().and_then(|message| {
            self.time.pop_front().map(|time| (message, time))
        })
    }

    pub fn peek(&self) -> Option<(&Message, &Duration, &bool)> {
        let message = self.messages.front()?;
        let time = self.time.front()?;
        let is_special = self.is_special.front()?;
        Some((message, time, is_special))
    }

    pub fn log(&mut self, message: Message, time: Duration, is_special: bool) {
        self.messages.push_back(message);
        self.time.push_back(time);
        self.is_special.push_back(is_special);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.time.clear();
        self.is_special.clear();
    }
}