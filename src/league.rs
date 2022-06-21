use crate::comp::*;
use crate::game_data::{GameData, GameDatum};
use crate::java_random::Random;
use crate::teams::{Teams, Division};

const SEED: i64 = -4;

#[derive(Clone, Debug)]
pub struct League {
    pub rng: Random,

    pub players: Players,
    pub teams: Teams,
    pub game_data: GameData,
    pub game_results: GameResults,
}

impl League {
    pub fn new(players: Players, teams: Teams, game_data: GameData, game_results: GameResults) -> Self {
        League {
            rng: Random::new(SEED),
            players,
            teams,
            game_data,
            game_results
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
        self.teams.clear();
        self.game_data.clear();
        self.game_results.clear();
    }

    pub fn initialize_teams(&mut self) {
        let teams = vec![
            ("Bluetooth", "Hellmouth", "📶", "BLT"),
            ("Rollers","Vegas","👁","VGS"),
            ("Minimalists","Minnesota","🗤","MIN"),
            ("Chorale","Cosquin","🎼","CHR"),
            ("Greetings","Green Bank","📡","GRN"),
            ("Bluescreens","California","💻","CLF"),
            ("Boas","Myanmar","🐍","BOA"),
            ("Candy Floss","England","🍭","ENG"),
            ("Pears","Portland","🍐","PRT"),
            ("Champions","Calgary","🏆","CLG"),
            ("Zoomers","Detroit","🚄","DTR"),
            ("Fate","Columbus","⚜","COL"),
            ("Lettuces","Romania","🌱","RMN"),
            ("Metalheads","Gaborone","🔩","GBR"),
            ("Labs","Labrador","🔬","LAB"),
            ("Dams","Hoover","🌉","DAM"),
            ("Pufferfish","Pacific","🐡","PUF"),
            ("Limes","Key West","🔑","KEY"),
            ("Toboggans","Trinidad","❄","TRI"),
            ("Rice-cakes","Reno","🍘","RNO"),
            ("Cool Guys","Colorado","🕶","COL"),
            ("Squares","Egyptian","🔲","EGP"),
            ("Bluegrass","Kentucky","📻","KNT"),
            ("Crepes","French","🥞","FRC")
        ];

        // Note that this will all become unecessary when we have a proper database
        let mut mod_six = 0;
        for (i, team) in teams.iter().enumerate() {
            let (name, city, emoji, abbreviation) = team;
            let division = {
                let mod_six_value = mod_six;
                if i % 6 == 0 {
                    mod_six += 1;
                };
                match mod_six_value {
                    0 => Division::UltraDark,
                    1 => Division::ModerateDark,
                    2 => Division::UltraLight,
                    3 => Division::ModerateLight,
                    _ => panic!("Unreachable!")
                }
            };

            self.teams.create_team(name, city, emoji, abbreviation, division);
        }
    }

    pub fn find_by_name() {}


    // Wait this won't work. I'll figure it out later
    pub fn find_game_results_by_day(&self, day: u32) -> Vec<GameResult> {
        self.game_results.data.into_iter()
            .filter(|&game| game.day == day)
            .collect()
    }
}

impl Default for League {
    fn default() -> Self {
        League {
            players: Players::default(),
            teams: Teams::default(),
            game_data: GameData::default(),
            game_results: GameResults::default(),
        }
    }
}