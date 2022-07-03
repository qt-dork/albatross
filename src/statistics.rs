
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Statistic {
    /// Wins (W) Pitcher stat???
    Wins, // panda approved
    /// Losses (L) Pitcher Stat???
    Losses,
    /// Win-Loss Percentage (W-L%)
    WinLossPercentage,
    // Earned Run Average (ERA) Pitcher stat
    EarnedRunAverage,
    /// Games Played (G) Pitcher and batter stat
    GamesPlayed, // panda approved
    /// Shutouts (SHO) Pitcher stat
    Shutouts, // panda approved
    /// Saves (SV)
    Saves,
    /// Innings Pitched (IP) Pitcher stat
    InningsPitched, // panda approved
    /// Hits Allowed (H) Pitcher stat
    HitsAllowed,
    /// Runs Allowed (R) Pitcher stat
    RunsAllowed, // panda approved
    /// Home Runs Allowed (HR) Pitcher stat
    HomeRunsAllowed, // panda approved
    /// Bases on Balls aka Walks (BB) Pitcher    stat
    BasesOnBalls, // panda approved
    /// Bases on Balls aka Walks (BB) Batter stat
    Walks, // panda approved
    /// Strikeouts (SO) Pitcher stat - The number of times a pitcher has struck a batter out.
    ///
    /// This has to be separated from Strikeouts for pitchers because if a batter were to become a pitcher, then this would work differently
    StrikeoutsPitcher, // panda approved
    /// Strikeouts (SO) Batter stat - The number of times a batter has been struck out.
    /// 
    /// This has to be separated from Strikeouts for pitchers because if a batter were to become a pitcher, then this would work differently
    StrikeoutsBatter, // panda approved
    /// Batters Faced (BF) Pitcher stat
    BattersFaced,
    /// Adjusted ERA (100 is league average) (ERA+) Pitcher stat
    AdjustedERA,
    /// Fielding Independent Pitching (FIP) Pitcher stat
    FieldingIndependentPitching,
    /// Hits per 9 Innings (HR/9) Pitcher stat
    HitsPer9Innings,
    /// Home Runs per 9 Innings (HR/9) Pitcher stat
    HomeRunsPer9Innings,
    /// Walks per 9 Innings (BB/9) Pitcher stat
    WalksPer9Innings,
    /// Strikeouts per 9 Innings (SO/9) Pitcher stat
    StrikeoutsPer9Innings,
    /// Strikeout-to-Walk Ratio (SO/BB) Pitcher stat
    StrikeoutToWalkRatio,
    /// Plate Appearances (PA) Batter stat
    PlateAppearances, // panda approved
    /// At-Bats (AB) Batter stat
    AtBats,
    /// Runs Scored (R) Batter stat
    RunsScored, // panda approved
    /// Hits (H) Batter stat
    Hits, // panda approved
    /// Doubles (2B) Batter stat
    Doubles, // panda approved
    /// Triples (3B) Batter stat
    Triples, // panda approved
    /// Home Runs (HR) Batter stat
    HomeRuns, // panda approved
    /// Runs Batted In (RBI) Batter stat
    RunsBattedIn,
    /// Stolen Bases (SB) Batter stat
    StolenBases, // panda approved
    /// Caught Stealing (CS) Batter stat
    CaughtStealing, // panda approved
    /// Batting Average (BA) Batter stat
    BattingAverage,
    /// On-base Percentage (OBP) Batter stat
    OnBasePercentage,
    /// Slugging Percentage (SLG) Batter stat
    SluggingPercentage,
    /// On-base Plus Slugging (OPS) Batter stat
    OnBasePlusSlugging,
    /// Adjusted OPS (100 is league average) (OPS+) Batter stat
    AdjustedOPS,
    /// Batting Average on Balls In Play (BABIP) Batter stat
    BattingAverageOnBallsInPlay,
    /// Total Bases (TB) Batter stat
    TotalBases,
    /// Double Plays Grounded Into (GIDP) Batter stat
    DoublePlaysGroundedInto,
    /// Sacrifices (SAC) Batter stat
    Sacrifices, // I straight up don't know if this can be implemented?

    // Proposed stats
    // Foul Balls Hit (FBS) Batter stat
    // FoulBallsHit, // Idk if this can be implemented
    // Flyouts Hit (FHL) Batter stat
    // FlyoutsHit, // Idk if this can be implemented
    // Groundouts Hit (GHL) Batter stat
    // GroundoutsHit, // Idk if this can be implemented
    // Base Steal Attempts (BSA) Batter stat
    BaseStealAttempts,
    // Second Base Stolen (SBS) Batter stat
    // SecondBaseStolen,
    // Caught Stealing Second Base (CSBS) Batter stat
    // CaughtStealingSecondBase,
    // Third Base Stolen (TBS) Batter stat
    // ThirdBaseStolen,
    // Caught Stealing Third Base (CSBS) Batter stat
    // CaughtStealingThirdBase,
    // Home Stolen (HST) Batter stat
    // HomeStolen,
    // Caught Stealing Home (CSHT) Batter stat
    // CaughtStealingHome,
    // Times Pitched To (TPT) Batter stat
    // TimesPitchedTo,
    // Strikes (STK) Batter stat
    // Strikes,
    // Swinging Strikes (SSTK) Batter stat
    // SwingingStrikes,
    // Looking Strikes (LSTK) Batter stat
    // LookingStrikes,
    // Balls Received (BRC) Batter stat
    // BallsReceived,
    // Flyouts Caught (FCC) Fielding stat
    // FlyoutsCaught,
    // Ground Outs Caught (GCC) Fielding stat
    // GroundOutsCaught,
    // Pitches Thrown (PTH) Pitcher stat
    // PitchesThrown,
    // Balls Thrown (BTH) Pitcher stat
    // BallsThrown, // Wait is this different from balls because it implies intent? This doesn't work with the current system because balls are not based on intent but instead outcome
    // Foul Balls Pitched (FBP) Pitcher stat
    // FoulBallsThrown,
    // Singles Allowed (1B) Pitcher stat
    // SinglesAllowed,
    // Doubles Allowed (2B) Pitcher stat
    // DoublesAllowed,
    // Triples Allowed (3B) Pitcher stat
    // TriplesAllowed,
}

impl Statistic {
    pub fn shorthand(&self) -> &str {
        match self {
            Statistic::Wins => "W",
            Statistic::Losses => "L",
            Statistic::WinLossPercentage => "W-L%",
            Statistic::EarnedRunAverage => "ERA",
            Statistic::GamesPlayed => "G",
            Statistic::Shutouts => "SHO",
            Statistic::Saves => "SV",
            Statistic::InningsPitched => "IP",
            Statistic::HitsAllowed => "H",
            Statistic::RunsAllowed => "R",
            Statistic::HomeRunsAllowed => "HR",
            Statistic::BasesOnBalls => "BB",
            Statistic::Walks => "BB",
            Statistic::StrikeoutsPitcher => "SO",
            Statistic::StrikeoutsBatter => "SO",
            Statistic::BattersFaced => "BF",
            Statistic::AdjustedERA => "ERA+",
            Statistic::FieldingIndependentPitching => "FIP",
            Statistic::HitsPer9Innings => "HR/9",
            Statistic::HomeRunsPer9Innings => "HR/9",
            Statistic::WalksPer9Innings => "BB/9",
            Statistic::StrikeoutsPer9Innings => "SO/9",
            Statistic::StrikeoutToWalkRatio => "SO/BB",
            Statistic::PlateAppearances => "PA",
            Statistic::AtBats => "AB",
            Statistic::RunsScored => "R",
            Statistic::Hits => "H",
            Statistic::Doubles => "2B",
            Statistic::Triples => "3B",
            Statistic::HomeRuns => "HR",
            Statistic::RunsBattedIn => "RBI",
            Statistic::StolenBases => "SB",
            Statistic::CaughtStealing => "CS",
            Statistic::BattingAverage => "BA",
            Statistic::OnBasePercentage => "OBP",
            Statistic::SluggingPercentage => "SLG",
            Statistic::OnBasePlusSlugging => "OPS",
            Statistic::AdjustedOPS => "OPS+",
            Statistic::BattingAverageOnBallsInPlay => "BABIP",
            Statistic::TotalBases => "TB",
            Statistic::DoublePlaysGroundedInto => "GIDP",
            Statistic::Sacrifices => "SAC",
            Statistic::BaseStealAttempts => "BSA",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Statistic::Wins => "Wins",
            Statistic::Losses => "Losses",
            Statistic::WinLossPercentage => "Win-Loss Percentage",
            Statistic::EarnedRunAverage => "Earned Run Average",
            Statistic::GamesPlayed => "Games Played",
            Statistic::Shutouts => "Shutouts",
            Statistic::Saves => "Saves",
            Statistic::InningsPitched => "Innings Pitched",
            Statistic::HitsAllowed => "Hits Allowed",
            Statistic::RunsAllowed => "Runs Allowed",
            Statistic::HomeRunsAllowed => "Home Runs Allowed",
            Statistic::BasesOnBalls => "Bases on Balls",
            Statistic::Walks => "Walks",
            Statistic::StrikeoutsPitcher => "Strikeouts",
            Statistic::StrikeoutsBatter => "Strikeouts",
            Statistic::BattersFaced => "Batters Faced",
            Statistic::AdjustedERA => "Adjusted Earned Run Average",
            Statistic::FieldingIndependentPitching => "Fielding Independent Pitching",
            Statistic::HitsPer9Innings => "Hits per 9 Innings",
            Statistic::HomeRunsPer9Innings => "Home Runs per 9 Innings",
            Statistic::WalksPer9Innings => "Walks per 9 Innings",
            Statistic::StrikeoutsPer9Innings => "Strikeouts per 9 Innings",
            Statistic::StrikeoutToWalkRatio => "Strikeout to Walk Ratio",
            Statistic::PlateAppearances => "Plate Appearances",
            Statistic::AtBats => "At Bats",
            Statistic::RunsScored => "Runs Scored",
            Statistic::Hits => "Hits",
            Statistic::Doubles => "Doubles",
            Statistic::Triples => "Triples",
            Statistic::HomeRuns => "Home Runs",
            Statistic::RunsBattedIn => "Runs Batted In",
            Statistic::StolenBases => "Stolen Bases",
            Statistic::CaughtStealing => "Caught Stealing",
            Statistic::BattingAverage => "Batting Average",
            Statistic::OnBasePercentage => "On Base Percentage",
            Statistic::SluggingPercentage => "Slugging Percentage",
            Statistic::OnBasePlusSlugging => "On Base Plus Slugging",
            Statistic::AdjustedOPS => "Adjusted On Base Plus Slugging",
            Statistic::BattingAverageOnBallsInPlay => "Batting Average on Balls in Play",
            Statistic::TotalBases => "Total Bases",
            Statistic::DoublePlaysGroundedInto => "Double Plays Grounded Into",
            Statistic::Sacrifices => "Sacrifices",
            Statistic::BaseStealAttempts => "Base Steal Attempts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct Statistics {
    pub wins: u32,
    pub losses: u32,
    pub win_loss_percentage: f64,
    pub earned_run_average: f64,
    pub games_played: u32,
    pub shutouts: u32,
    pub saves: u32,
    pub innings_pitched: u32,
    pub hits_allowed: u32,
    pub runs_allowed: u32,
    pub home_runs_allowed: u32,
    pub bases_on_balls: u32,
    pub walks: u32,
    pub strikeouts_pitcher: u32,
    pub strikeouts_batter: u32,
    pub batters_faced: u32,
    pub adjusted_era: f64,
    pub fielding_independent_pitching: f64,
    pub hits_per_9_innings: f64,
    pub home_runs_per_9_innings: f64,
    pub walks_per_9_innings: f64,
    pub strikeouts_per_9_innings: f64,
    pub strikeout_to_walk_ratio: f64,
    pub plate_appearances: u32,
    pub at_bats: u32,
    pub runs_scored: u32,
    pub hits: u32,
    pub doubles: u32,
    pub triples: u32,
    pub home_runs: u32,
    pub runs_batted_in: u32,
    pub stolen_bases: u32,
    pub caught_stealing: u32,
    pub batting_average: f64,
    pub on_base_percentage: f64,
    pub slugging_percentage: f64,
    pub on_base_plus_slugging: f64,
    pub adjusted_ops: f64,
    pub batting_average_on_balls_in_play: f64,
    pub total_bases: u32,
    pub double_plays_grounded_into: u32,
    pub sacrifices: u32,
    pub base_steal_attempts: u32,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            wins: 0,
            losses: 0,
            win_loss_percentage: 0.0,
            earned_run_average: 0.0,
            games_played: 0,
            shutouts: 0,
            saves: 0,
            innings_pitched: 0,
            hits_allowed: 0,
            runs_allowed: 0,
            home_runs_allowed: 0,
            bases_on_balls: 0,
            walks: 0,
            strikeouts_pitcher: 0,
            strikeouts_batter: 0,
            batters_faced: 0,
            adjusted_era: 0.0,
            fielding_independent_pitching: 0.0,
            hits_per_9_innings: 0.0,
            home_runs_per_9_innings: 0.0,
            walks_per_9_innings: 0.0,
            strikeouts_per_9_innings: 0.0,
            strikeout_to_walk_ratio: 0.0,
            plate_appearances: 0,
            at_bats: 0,
            runs_scored: 0,
            hits: 0,
            doubles: 0,
            triples: 0,
            home_runs: 0,
            runs_batted_in: 0,
            stolen_bases: 0,
            caught_stealing: 0,
            batting_average: 0.0,
            on_base_percentage: 0.0,
            slugging_percentage: 0.0,
            on_base_plus_slugging: 0.0,
            adjusted_ops: 0.0,
            batting_average_on_balls_in_play: 0.0,
            total_bases: 0,
            double_plays_grounded_into: 0,
            sacrifices: 0,
            base_steal_attempts: 0,
        }
    }

    pub fn clear(&mut self) {
        *self = Statistics::new();
    }

    pub fn add_to(&mut self, statistic: Statistic) {
        self.add_to_by(statistic, 1.0);
    }

    pub fn add_to_by(&mut self, statistic: Statistic, amount: f64) {
        // Not all of these are going to be stats you actually add to, but I'll handle that later
        match statistic {
            Statistic::Wins => self.wins += amount as u32,
            Statistic::Losses => self.losses += amount as u32,
            Statistic::WinLossPercentage => self.win_loss_percentage += amount,
            Statistic::EarnedRunAverage => self.earned_run_average += amount,
            Statistic::GamesPlayed => self.games_played += amount as u32,
            Statistic::Shutouts => self.shutouts += amount as u32,
            Statistic::Saves => self.saves += amount as u32,
            Statistic::InningsPitched => self.innings_pitched += amount as u32,
            Statistic::HitsAllowed => self.hits_allowed += amount as u32,
            Statistic::RunsAllowed => self.runs_allowed += amount as u32,
            Statistic::HomeRunsAllowed => self.home_runs_allowed += amount as u32,
            Statistic::BasesOnBalls => self.bases_on_balls += amount as u32,
            Statistic::Walks => self.walks += amount as u32,
            Statistic::StrikeoutsPitcher => self.strikeouts_pitcher += amount as u32,
            Statistic::StrikeoutsBatter => self.strikeouts_batter += amount as u32,
            Statistic::BattersFaced => self.batters_faced += amount as u32,
            Statistic::AdjustedERA => self.adjusted_era += amount,
            Statistic::FieldingIndependentPitching => {
                self.fielding_independent_pitching += amount
            }
            Statistic::HitsPer9Innings => self.hits_per_9_innings += amount,
            Statistic::HomeRunsPer9Innings => self.home_runs_per_9_innings += amount,
            Statistic::WalksPer9Innings => self.walks_per_9_innings += amount,
            Statistic::StrikeoutsPer9Innings => self.strikeouts_per_9_innings += amount,
            Statistic::StrikeoutToWalkRatio => self.strikeout_to_walk_ratio += amount,
            Statistic::PlateAppearances => self.plate_appearances += amount as u32,
            Statistic::AtBats => self.at_bats += amount as u32,
            Statistic::RunsScored => self.runs_scored += amount as u32,
            Statistic::Hits => self.hits += amount as u32,
            Statistic::Doubles => self.doubles += amount as u32,
            Statistic::Triples => self.triples += amount as u32,
            Statistic::HomeRuns => self.home_runs += amount as u32,
            Statistic::RunsBattedIn => self.runs_batted_in += amount as u32,
            Statistic::StolenBases => self.stolen_bases += amount as u32,
            Statistic::CaughtStealing => self.caught_stealing += amount as u32,
            Statistic::BattingAverage => self.batting_average += amount,
            Statistic::OnBasePercentage => self.on_base_percentage += amount,
            Statistic::SluggingPercentage => self.slugging_percentage += amount,
            Statistic::OnBasePlusSlugging => self.on_base_plus_slugging += amount,
            Statistic::AdjustedOPS => self.adjusted_ops += amount,
            Statistic::BattingAverageOnBallsInPlay => {
                self.batting_average_on_balls_in_play += amount
            }
            Statistic::TotalBases => self.total_bases += amount as u32,
            Statistic::DoublePlaysGroundedInto => {
                self.double_plays_grounded_into += amount as u32
            }
            Statistic::Sacrifices => self.sacrifices += amount as u32,
            Statistic::BaseStealAttempts => self.base_steal_attempts += amount as u32,
        }
    }

    /// Updates derived statistics (i.e. statistics based off of other statistics)
    fn update_stats(&mut self) {}

    /// Formats every single stat into an easy to display block of text.
    fn to_string(&self) -> String {
        // TODO: Actually make this
        "".to_string()
    }
}