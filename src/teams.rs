use crate::comp::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Teams {
    pub id: Vec<TeamId>,
    next_id: TeamId,

    pub name: Comp<String>,
    pub location: Comp<String>,
    pub logo: Comp<String>,
    pub abbreviation: Comp<String>,

    pub division: Comp<Division>,

    pub lineup: Comp<Vec<PlayerId>>,
    pub rotation: Comp<Vec<PlayerId>>,

    pub non_losses: Comp<u32>,
    pub wins: Comp<i32>,
    pub losses: Comp<i32>,
    pub favor: Comp<u32>,

    // Eventually teams may contain modifications
}

impl Teams {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        *self = Default::default()
    }

    pub fn create_team(&mut self, 
        name: &str, 
        location: &str, 
        logo: &str,
        abbreviation: &str,
        division: Division,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.id.push(id);
        
        self.name.insert(id, name.to_string());
        self.location.insert(id, location.to_string());
        self.logo.insert(id, logo.to_string());
        self.abbreviation.insert(id, abbreviation.to_string());
        self.division.insert(id, division);

        self.non_losses.insert(id, 0);
        self.wins.insert(id, 0);
        self.losses.insert(id, 0);

        self.favor.insert(id, id.try_into().unwrap());
    }
}

impl Default for Teams {
    fn default() -> Self {
        Teams {
            id: Vec::new(),
            next_id: 0,

            name: Comp::new(),
            location: Comp::new(),
            logo: Comp::new(),
            abbreviation: Comp::new(),

            division: Comp::new(),

            lineup: Comp::new(),
            rotation: Comp::new(),

            non_losses: Comp::new(),
            wins: Comp::new(),
            losses: Comp::new(),
            favor: Comp::new(),
        }
    }
}

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