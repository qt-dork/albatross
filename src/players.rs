use crate::comp::*;
use crate::player_flavor::PregameRitual;
use crate::player_stats::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Players {
    pub id: Vec<PlayerId>,
    
    pub name: Comp<String>,
    pub team: Comp<TeamId>,
    pub original_team: Comp<TeamId>,

    pub aggression: Comp<PlayerStat>,
    pub arrogance: Comp<PlayerStat>,
    pub carcinization: Comp<PlayerStat>,
    pub damage: Comp<PlayerStat>,
    pub density: Comp<PlayerStat>,
    pub dexterity: Comp<PlayerStat>,
    pub dimensions: Comp<PlayerStat>,
    pub effort: Comp<PlayerStat>,
    pub focus: Comp<PlayerStat>,
    pub fun: Comp<PlayerStat>,
    pub grit: Comp<PlayerStat>,
    pub hit_points: Comp<PlayerStat>,
    pub malleability: Comp<PlayerStat>,
    pub mathematics: Comp<PlayerStat>,
    pub number_of_eyes: Comp<PlayerStat>,
    pub pinpointedness: Comp<PlayerStat>,
    pub powder: Comp<PlayerStat>,
    pub rejection: Comp<PlayerStat>,
    pub splash: Comp<PlayerStat>,
    pub wisdom: Comp<PlayerStat>,

    pub pregame_ritual: Comp<PregameRitual>,
    pub coffee_style: Comp<CoffeeStyle>,
    pub blood_type: Comp<BloodType>,
    pub fate: Comp<Fate>,
    pub soulscream: Comp<Soulscream>,

    pub statistics: Comp<Statistics>,
    pub modifications: Comp<Modifications>,
}

impl Players {
    pub fn is_dead() -> bool {
        // TODO
    }
}