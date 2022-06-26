use crate::comp::*;
use crate::java_random::Random;
use crate::mixed_name_gen::MixedNameGenerator;
use crate::player_stat::*;
use crate::attributes::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Players {
    pub id: Vec<PlayerId>,
    next_id: PlayerId,
    
    pub name: Comp<String>,
    pub team: Comp<TeamId>,
    pub original_team: Comp<TeamId>,

    pub attributes: Comp<Attributes>,

    pub deceased: Comp<bool>,

    // pub pregame_ritual: Comp<PregameRitual>,
    // pub coffee_style: Comp<CoffeeStyle>,
    // pub blood_type: Comp<BloodType>,
    // pub fate: Comp<Fate>,
    // pub soulscream: Comp<Soulscream>,

    // pub statistics: Comp<Statistics>,
    // pub modifications: Comp<Modifications>,
}

impl Players {
    pub fn create_player(&mut self, rng: &mut Random, team_id: TeamId) -> PlayerId {
        let id = self.next_id;
        self.next_id += 1;
        self.id.push(id);

        let mut gen = MixedNameGenerator::new(rng.seed());
        let range = gen.random(1, 5) + gen.random(1, 5);
        let name = gen.next_name_with_distribution() + " " + &gen.distribution_with_length(range);
        rng.set_seed(gen.seed());

        // insert stuff
        self.name.insert(id, name);
        self.team.insert(id, team_id);
        self.original_team.insert(id, team_id);
        self.attributes.insert(id, Attributes::random(rng));
        self.deceased.insert(id, false);

        id
    }

    pub fn is_dead(&self, id: PlayerId) -> bool {
        self.deceased[&id]
    }
}

impl Default for Players {
    fn default() -> Self {
        Players { 
            id: Vec::new(),
            next_id: 0,
            
            name: Comp::default(),
            team: Comp::default(),
            original_team: Comp::default(),
            attributes: Comp::default(),
            deceased: Comp::default(),
        }
    }
}