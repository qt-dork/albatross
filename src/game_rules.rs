use crate::league::*;
use crate::player::*;
use crate::java_random::Random;
use crate::game_data::*;
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