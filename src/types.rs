#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Division {
    UltraDark,
    ModerateDark,
    UltraLight,
    ModerateLight,
}

impl Division {
    pub fn flip_league(&self) -> Division {
        match self {
            Division::UltraDark => Division::ModerateDark,
            Division::ModerateDark => Division::UltraDark,
            Division::UltraLight => Division::ModerateDark,
            Division::ModerateLight => Division::UltraLight,
        }
    }
    pub fn flip_division(&self) -> Division {
        match self {
            Division::UltraDark => Division::UltraLight,
            Division::ModerateDark => Division::ModerateLight,
            Division::UltraLight => Division::UltraDark,
            Division::ModerateLight => Division::ModerateDark,
        }
    }

    pub fn is_dark(&self) -> bool {
        match self {
            Division::UltraDark => true,
            Division::ModerateDark => true,
            Division::UltraLight => false,
            Division::ModerateLight => false,
        }
    }

    pub fn is_light(&self) -> bool {
        match self {
            Division::UltraDark => false,
            Division::ModerateDark => false,
            Division::UltraLight => true,
            Division::ModerateLight => true,
        }
    }

    pub fn is_ultra(&self) -> bool {
        match self {
            Division::UltraDark => true,
            Division::ModerateDark => false,
            Division::UltraLight => true,
            Division::ModerateLight => false,
        }
    }

    pub fn is_moderate(&self) -> bool {
        match self {
            Division::UltraDark => false,
            Division::ModerateDark => true,
            Division::UltraLight => false,
            Division::ModerateLight => true,
        }
    }
}

pub enum Position {
    Lineup,
    Rotation
}

pub enum Pitching {
    Batter,
    Pitcher
}

pub enum PitchOutcome {
    Ball,
    // Can get a strikeout but the pitch shouldn't handle this imo?
    StrikeLooking,
    StrikeSwinging,
    FoulBall,
    Flyout,
    GroundOut,
    /// Number of bases hit.
    Hit(u8)
}

pub enum StealOutcome {
    /// The player has successfully stolen the base
    Steal,
    /// The player was caught stealing
    CaughtStealing,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameKind {
    /// This game affects regular season standings
    Regular,
    /// This is the first set of postseason games
    Postseason1,
    /// This is the second set of postseason games
    Postseason2,
    /// This is the third set of postseason games
    Postseason3,
    /// This is some sort of special game that has no standing on any other events
    Special,
}

impl Default for GameKind {
    fn default() -> Self {
        GameKind::Regular
    }
}