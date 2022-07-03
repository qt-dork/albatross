use crate::league::*;
use crate::player::*;
use crate::java_random::Random;
use crate::game_data::*;
use crate::statistics::Statistic;
use crate::comp::*;
use crate::types::*;

// TODO: Make return `Result`
/// Sets the deceased flag to true for the given player, and then replaces the player with a new player. Returns the new player's id.
/// Assumes that the player being passed into it is currently on a team.
pub fn kill_player(league: &mut League, player_id: PlayerId) -> PlayerId {
    league.players.deceased[&player_id] = true;
    let (position, team) = league.teams.contains(player_id).unwrap();
    let new_player = league.players.create_player(&mut league.rng, team);
    match position {
        // Replaces the dead player on the lineup with a new player.
        Position::Lineup => {
            league.teams.lineup.insert(team.try_into().unwrap(),
                league.teams.lineup
                    .get(&team)
                    .unwrap()
                    .iter()
                    .map(|player|
                        if *player == player_id {
                            new_player
                        } else {
                            *player
                        }
                    )
                    .collect()
            );
        }
        // Same thing for the rotation.
        Position::Rotation => {
            league.teams.rotation.insert(team.try_into().unwrap(),
                league.teams.rotation
                    .get(&team)
                    .unwrap()
                    .iter()
                    .map(|player|
                        if *player == player_id {
                            new_player
                        } else {
                            *player
                        }
                    )
                    .collect()
            );
        }
    }
    // TODO: Update player in game data as well

    new_player
}

/// Advances the bases by one, and returns Some(Player) if a player reached home, None if not.
pub fn advance_bases(bases: &mut [Option<PlayerId>]) -> Option<PlayerId> {
    bases.rotate_right(1);
    match bases[0] {
        Some(player_id) => {
            bases[0] = None;
            Some(player_id)
        }
        None => None,
    }
}

// changed from player to position because it's possible that the same player can occupy multiple bases
pub fn advance_base(bases: &[Option<PlayerId>], position: usize) -> (Vec<Option<PlayerId>>, Option<PlayerId>) {
    let mut new_bases = bases.to_vec();
    let player = new_bases[position].take();

    // advances the bases
    new_bases.rotate_left(1);
    if new_bases[position].is_none() {
        new_bases[position] = player;
    } else {
        panic!("Tried to advance base to a position that is already occupied");
    }
    new_bases.rotate_right(1);

    // checks if the player reached home
    if new_bases[0].is_some() {
        let player = new_bases[0].take();
        (new_bases, player)
    } else {
        (new_bases, None)
    }
}

/// Inserts a player into first base, advancing every player afterwards by one. 
/// If that results in a player reaching home, then that player is returned
pub fn push_front_bases(bases: &mut [Option<PlayerId>], player_id: PlayerId) -> Option<PlayerId> {
    let player = advance_bases(bases);
    bases[0] = Some(player_id);
    player
}

pub fn random_defender(league: &League, team: TeamId) -> Option<PlayerId> {
    let mut rng = league.rng;
    let defenders = league.teams.lineup.get(&team)?;
    
    Some(defenders[(rng.next_u32() % defenders.len() as u32) as usize])
}

pub fn random_baserunner(rng: &mut Random, bases: &[Option<PlayerId>]) -> Option<PlayerId> {
    let occupied_bases: Vec<PlayerId> = bases.iter().filter_map(|base| *base).collect();
    if occupied_bases.is_empty() {
        None
    } else {
        Some(occupied_bases[(rng.next_u32() % occupied_bases.len() as u32) as usize])
    }
}

/// This function runs on a successful urge check to see if the baserunner even wants to try to steal a base.
/// 
pub fn steal(league: &League, rng: &mut Random, player: &PlayerId, defender: &PlayerId) -> Option<StealOutcome> {
    // Get the player's attributes for use in the steal
    let player_attributes = league.players.attributes.get(player)?;
    let defender_attributes = league.players.attributes.get(defender)?;

    let steal_value = rng.next_f64() * 10. + player_attributes.dexterity.value();
    let defense_value = rng.next_f64() * 10. + defender_attributes.wisdom.value();

    if steal_value > defense_value {
        return Some(StealOutcome::Steal);
    } else {
        return Some(StealOutcome::CaughtStealing);
    }
}

pub fn attempt_steal(league: &League, rng: &mut Random, player: &PlayerId, defender: &PlayerId) -> Option<StealOutcome> {
    let player_arrogance = league.players.attributes.get(player)?.arrogance.value();
    let defender_rejection = league.players.attributes.get(defender)?.rejection.value();

    let urge = rng.next_f64() * 10. + player_arrogance / 3. - defender_rejection / 3.;
    if urge >= 9.95 {
        return steal(league, rng, player, defender);
    } else {
        return None;
    }
}

pub fn handle_steal(league: &League, rng: &mut Random, bases: &[Option<PlayerId>], defender: &PlayerId) -> Vec<Option<StealOutcome>> {
    bases.iter().enumerate().map(|(i, &base)| {
        let player_id = base?;
        if bases.get(i + 1)?.is_none() {
            attempt_steal(league, rng, &player_id, defender)
        } else {
            None
        }
    }).collect()
}

pub fn process_steal_outcome(league: &mut League, game_state: &GameDatum, bases: &[Option<PlayerId>], steal_outcomes: &[Option<StealOutcome>]) {
    steal_outcomes.into_iter().for_each(|steal_outcome| {
        if steal_outcome.is_some() {

        }
    });
}

pub fn clear_base(bases: &[Option<PlayerId>], player: &PlayerId) -> Vec<Option<PlayerId>> {
    bases.iter()
        .map(|option_base|
            if let Some(base) = option_base {
                if base == player {
                    None
                } else {
                    Some(*base)
                }
            } else {
                None
            })
        .collect()
}