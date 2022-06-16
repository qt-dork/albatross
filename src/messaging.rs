/// This handles all the messaging between the game and the user.
/// This is directly inspired by RustRoguelike's messaging system, which you can find here:
/// https://github.com/nsmryan/RustRoguelike/blob/master/roguelike_core/src/messaging.rs

use std::collections::VecDeque;

pub type BallsStrikes = (i32, i32);

// Also derives Copy, Deserialize, and Serialize
// Copy doesn't work here because we store player names instead of ids to players, whose names are retrieved from the database.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    AnnounceMatchup(String, String), // Team 1, Team 2
    StartGame,
    InningStart(bool, u32, String, String), // Top, Inning, Team1, Team2
    CurrentScore(String, f64, f64, String), // Team1Abbr, Team1Score, Team2Score, Team2Abbr
    Steal(String, usize), // Stealer Name, Base Stolen
    CaughtStealing(String, usize), // Stealer Name, Base Stolen
    Walk(String), // Batter Name
    Ball(BallsStrikes), // BallsStrikes
    StruckOutLooking(String, BallsStrikes), // Batter Name, Balls Strikes
    StrikeLooking(BallsStrikes), // Balls Strikes
    StruckOutSwinging(String, BallsStrikes), // Batter Name, Balls Strikes
    StrikeSwinging(BallsStrikes), // Balls Strikes
    FoulBall(BallsStrikes), // Balls Strikes
    Flyout(String, String), // Batter Name, Defender Name
    Groundout(String, String), // Batter Name, Defender Name
    // NOTE: Maybe I should have Bases Hit be its own enum?
    Scores(String), // Batter Name
    Hit(String, usize), // Batter Name, Bases Hit
    // Figure something out for the format_balls_strikes() function
    NextBatter(String, String), // Batter Name, Team Name
    Out(i32), // Number of outs
    InningToOuting(u32), // Inning
    EndGameScore(String, f64, String, f64), // Team1Name, Team1Score, Team2Name, Team2Score
    GameOver,
}

impl Message {
    pub fn message_line(&self) -> String {
        match self {
            Message::AnnounceMatchup(team1, team2) => {
                format!("{} vs. {}", team1, team2)
            },
            Message::StartGame => {
                "Blay pall!".to_string()
            },
            Message::InningStart(top, inning, team1, team2) => {
                match top {
                    true => {
                        format!("Top of {}, {} batting. {} pitching.", inning, team1, team2)
                    },
                    false => {
                        format!("Bottom of {}, {} batting. {}pitching.", inning, team1, team2)
                    }
                }
            },
            Message::CurrentScore(team1, score1, score2, team2) => {
                format!("[Current score is {} {}-{} {}]", team1, Self::score_as_string(score1), Self::score_as_string(score2), team2)
            },
            Message::Steal(stealer, base) => {
                let stolen_base: String;
                match base {
                    0 => {
                        stolen_base = "second base".to_string();
                    },
                    1 => {
                        stolen_base = "third base".to_string();
                    },
                    2 => {
                        stolen_base = "home".to_string();
                    },
                    _ => {
                        // Fourth base isn't out yet
                        panic!("Invalid base number: {}", base);
                    }
                }
                format!("{} steals {}!", stealer, stolen_base)
            },
            Message::CaughtStealing(stealer, base) => {
                let stolen_base: String;
                match base {
                    0 => {
                        stolen_base = "second base".to_string();
                    },
                    1 => {
                        stolen_base = "third base".to_string();
                    },
                    2 => {
                        stolen_base = "home".to_string();
                    },
                    _ => {
                        // Fourth base isn't out yet
                        panic!("Invalid base number: {}", base);
                    }
                }
                format!("{} gets caught stealing {}.", stealer, stolen_base)
            },
            Message::Walk(batter) => {
                format!("{} draws a walk.", batter)
            },
            Message::Ball((balls, strikes)) => {
                format!("Ball. {}-{}", balls, strikes)
            },
            Message::StruckOutLooking(batter, balls_strikes) => {
                format!("{} strikes out looking. ", batter) + &Self::format_balls_strikes(balls_strikes)
            },
            Message::StrikeLooking(balls_strikes) => {
                "Strike, looking. ".to_owned() + &Self::format_balls_strikes(balls_strikes)
            },
            Message::StruckOutSwinging(batter, balls_strikes) => {
                format!("{} strikes out swinging. ", batter) + &Self::format_balls_strikes(balls_strikes)
            },
            Message::StrikeSwinging(balls_strikes) => {
                "Strike, swinging. ".to_owned() +  &Self::format_balls_strikes(balls_strikes)
            },
            Message::FoulBall(balls_strikes) => {
                "Foul ball. ".to_owned() + &Self::format_balls_strikes(balls_strikes)
            },
            Message::Flyout(batter, defender) => {
                format!("{} hit a flyout to {}.", batter, defender)
            },
            Message::Groundout(batter, defender) => {
                format!("{} hit a ground out to {}!", batter, defender)
            },
            Message::Scores(batter) => {
                format!("{} scores!", batter)
            },
            Message::Hit(batter, bases_hit) => {
                // let mut message = format!("{} his a ", batter);
                let base: String;
                match bases_hit {
                    1 => {
                        base = "Single".to_string();
                    },
                    2 => {
                        base = "Double".to_string();
                    },
                    3 => {
                        base = "Triple".to_string();
                    },
                    // Eventually this might need to support the fourth base, if that's ever added
                    _ => {
                        base = "Home Run".to_string();
                    }
                }
                format!("{} hits a {}!", batter, base)
            },
            Message::NextBatter(batter, team) => {
                format!("{} batting for the {}.", batter, team)
            },
            Message::Out(outs) => {
                format!("[Out {}]", outs)
            },
            Message::InningToOuting(inning) => {
                format!("Inning {} is now an Outing.", inning)
            },
            //DoneFix
            Message::EndGameScore(team1, wins1, team2, wins2) => {
                format!("{} {}, {} {}", team1, Self::score_as_string(wins1), Self::score_as_string(wins2), team2)
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
        format!("{}-{}", b, s)
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
#[derive(Debug, Clone)]
pub struct MessageLog {
    pub messages: VecDeque<Message>,
    pub time: VecDeque<u128>,
}

impl MessageLog {
    pub fn new() -> Self {
        MessageLog {
            messages: VecDeque::new(),
            time: VecDeque::new(),
        }
    }

    pub fn len(&self) -> Option<usize> {
        if self.messages.len() != self.time.len() {
            None
        } else {
            Some(self.messages.len())
        }
    }

    pub fn pop_front(&mut self) -> Option<(Message, u128)> {
        self.messages.pop_front().and_then(|message| {
            self.time.pop_front().map(|time| (message, time))
        })
    }

    pub fn peek(&self) -> Option<&Message> {
        self.messages.front()
    }

    pub fn log(&mut self, message: Message, time: u128) {
        self.messages.push_back(message);
        self.time.push_back(time);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.time.clear();
    }
}

