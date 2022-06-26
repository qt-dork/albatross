use crate::comp::*;
// use crate::game_data::{GameData, GameDatum};
use crate::java_random::Random;
use crate::teams::Teams;
use crate::players::*;
use crate::game_results::*;
use crate::types::*;

const SEED: i64 = -4;

#[derive(Clone, Debug)]
pub struct League {
    pub rng: Random,

    pub players: Players,
    pub teams: Teams,
    // pub game_data: GameData,
    pub game_results: Vec<GameResult>,
}

impl League {
    pub fn new() -> Self {
        League {
            rng: Random::new(SEED),
            players: Players::default(),
            teams: Teams::default(),
            // game_data: GameData::new(),
            game_results: Vec::new(),
        }
    }

    pub fn from(
        players: Players, 
        teams: Teams, 
        // game_data: GameData, 
        game_results: Vec<GameResult>
    ) -> Self {
        League {
            rng: Random::new(SEED),
            players,
            teams,
            // game_data,
            game_results
        }
    }

    pub fn clear(&mut self) {
        // self.players.clear();
        self.teams.clear();
        // self.game_data.clear();
        // self.game_results.clear();
    }

    pub fn initialize_teams(&mut self) {
        let teams = vec![
            ("Bluetooth", "Hellmouth", "📶", "BLT", Division::UltraDark),
            ("Rollers","Vegas","👁","VGS", Division::UltraDark),
            ("Minimalists","Minnesota","🗤","MIN", Division::UltraDark),
            ("Chorale","Cosquin","🎼","CHR", Division::UltraDark),
            ("Greetings","Green Bank","📡","GRN", Division::UltraDark),
            ("Bluescreens","California","💻","CLF", Division::UltraDark),
            ("Boas","Myanmar","🐍","BOA", Division::ModerateDark),
            ("Candy Floss","England","🍭","ENG", Division::ModerateDark),
            ("Pears","Portland","🍐","PRT", Division::ModerateDark),
            ("Champions","Calgary","🏆","CLG", Division::ModerateDark),
            ("Zoomers","Detroit","🚄","DTR", Division::ModerateDark),
            ("Fate","Columbus","⚜","COL", Division::ModerateDark),
            ("Lettuces","Romania","🌱","RMN", Division::UltraLight),
            ("Metalheads","Gaborone","🔩","GBR", Division::UltraLight),
            ("Labs","Labrador","🔬","LAB", Division::UltraLight),
            ("Dams","Hoover","🌉","DAM", Division::UltraLight),
            ("Pufferfish","Pacific","🐡","PUF", Division::UltraLight),
            ("Limes","Key West","🔑","KEY", Division::UltraLight),
            ("Toboggans","Trinidad","❄","TRI", Division::ModerateLight),
            ("Rice-cakes","Reno","🍘","RNO", Division::ModerateLight),
            ("Cool Guys","Colorado","🕶","COL", Division::ModerateLight),
            ("Squares","Egyptian","🔲","EGP", Division::ModerateLight),
            ("Bluegrass","Kentucky","📻","KNT", Division::ModerateLight),
            ("Crepes","French","🥞","FRC", Division::ModerateLight)
        ];

        // Note that this will all become unnecessary when we have a proper database
        teams.iter().for_each(|(name, location, logo, abbreviation, division)| {
            // Create Team
            let team = self.teams.create_team(name, location, logo, abbreviation, *division);

            // Create Players
            let lineup: Vec<PlayerId> = (0..9).map(|_| self.players.create_player(&mut self.rng, team)).collect();
            self.teams.lineup.insert(team, lineup);

            let rotation: Vec<PlayerId> = (0..5).map(|_| self.players.create_player(&mut self.rng, team)).collect();
            self.teams.rotation.insert(team, rotation);
        });
    }

    // pub fn find_team_by_name(&self, name: &str) -> Option<TeamId> {
    //     self.teams.name.
    // }

    // }


    // Wait this won't work. I'll figure it out later
    // pub fn find_game_results_by_day(&self, day: u32) -> Vec<GameResult> {
    //     self.game_results.data.into_iter()
    //         .filter(|&game| game.day == day)
    //         .collect()
    // }
}

impl Default for League {
    fn default() -> Self {
        League {
            rng: Random::new(SEED),
            players: Players::default(),
            teams: Teams::default(),
            // game_data: GameData::default(),
            game_results: Vec::default(),
        }
    }
}