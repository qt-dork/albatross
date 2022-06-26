use crate::comp::*;
use crate::types::{Division, Position};

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
    ) -> TeamId {
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

        id
    }

    /// Gets the PlayerId of every player on both the lineup and rotation.
    pub fn active_roster(&self, team: TeamId) -> Vec<PlayerId> {
        let mut roster = self.lineup.get(&team).unwrap().clone();
        roster.extend_from_slice(&self.rotation.get(&team).unwrap());
        roster
    }

    /// Checks which team a player is on. Returns the team that player is on, and the position they're in. Otherwise returns None.
    pub fn contains(&self, player_id: PlayerId) -> Option<(Position, TeamId)> {
        for team in self.id.iter() {
            if self.lineup.get(team).unwrap().contains(&player_id) {
                return Some((Position::Lineup, *team));
            }
            if self.rotation.get(team).unwrap().contains(&player_id) {
                return Some((Position::Rotation, *team));
            }
        }
        None
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