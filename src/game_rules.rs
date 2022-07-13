use crate::league::*;
use crate::messaging::Message;
use crate::outcomes::{StealOutcomes, StealOutcome};
use crate::player::*;
use crate::java_random::Random;
use crate::game_data::*;
use crate::statistics::Statistic;
use crate::comp::*;
use crate::types::*;
use crate::bases::Bases;

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

pub fn random_defender(league: &League, team: TeamId) -> Option<PlayerId> {
	let mut rng = league.rng;
	let defenders = league.teams.lineup.get(&team)?;
	
	Some(defenders[(rng.next_u32() % defenders.len() as u32) as usize])
}

pub fn random_baserunner(rng: &mut Random, bases: Bases) -> Option<PlayerId> {
	let occupied_bases: Vec<PlayerId> = bases.iter().filter_map(|base| *base).collect();
	if occupied_bases.is_empty() {
		None
	} else {
		Some(occupied_bases[rng.next_u32() as usize % occupied_bases.len()])
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

pub fn handle_steal(league: &League, rng: &mut Random, bases: &Bases, defender: &PlayerId) -> StealOutcomes {
	let mut steal_outcomes = StealOutcomes::new(bases.clone());

	steal_outcomes.steal_outcomes = bases.iter().enumerate().map(|(i, &base)| {
		let player_id = base;
		let mut outcomes = vec![];
		if let Some(player) = player_id {
			let mut i = i;
			loop {
				if bases.is_next_occupied(i) {
					let outcome = attempt_steal(league, rng, &player, defender);
					if let Some(outcome) = outcome {
						outcomes.push(outcome);
					}
					if outcome != Some(StealOutcome::Steal) {
						break;
					}
				} else {
					break;
				}
			}
		}
		outcomes
	}).collect();

	steal_outcomes
}

pub fn process_steal_outcome(league: &mut League, game_state: &mut GameDatum, bases: &Bases, steal_outcomes: StealOutcomes) -> Bases {
	let mut new_bases = bases.clone();

	// This function actually scores the players itself, so it means that it'll record runs and then score any of them
	let mut runs = Vec::new();

	// Useless variable. It's meant to make sure that processing stops if a player is caught stealing, but I don't know if blaseball even does it.
	let mut out_at = None;
	for (i, steal_outcome) in steal_outcomes.steal_outcomes.iter().enumerate().rev() {
		// Placeholder code if a player is caught stealing.
		if steal_outcome.len() > 0 { // && out_at.is_none() {
			steal_outcome.into_iter().for_each(|outcome| {
				match outcome {
					StealOutcome::Steal => {
						game_state.log(Message::Steal(bases[i].unwrap(), i + 1));
						let run = new_bases.advance_base(i);
						if let Some(run) = run {
							runs.push(run);
						}
					}
					StealOutcome::CaughtStealing => {
						game_state.log(Message::CaughtStealing(bases[i].unwrap(), i + 1));
						new_bases[i] = None;
						out_at = Some(i);
					}
				}
			});
		}
	}

	runs.into_iter().for_each(|player| {
		// TODO: Make score function
		// score(player);
	});
	
	new_bases
}

/// Removes a player from the bases.
/// This function probably isn't particularly useful, since it removes all instances of a player, even if they're on multiple bases.
/// Use `clear_base_number` instead if you're trying to remove a position from a base.
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

/// Removes a player from a specific base position.
/// An alternative to `clear_base` if a player appears on multiple bases or you're trying to remove a player by position instead of by player.
pub fn clear_base_number(bases: &[Option<PlayerId>], position: usize) -> Vec<Option<PlayerId>> {
	let mut new_bases = bases.to_vec();
	new_bases[position] = None;
	new_bases
}

// Removes every player from the bases.
pub fn clear_bases(bases: &[Option<PlayerId>]) -> Vec<Option<PlayerId>> {
	bases.iter()
		.map(|option_base|
			if let Some(base) = option_base {
				None
			} else {
				None
			})
		.collect()
}