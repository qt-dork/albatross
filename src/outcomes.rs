use crate::{bases::Bases, comp::PlayerId};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StealOutcome {
  /// The player has successfully stolen the base
  Steal,
  /// The player was caught stealing
  CaughtStealing,
}

/// Complex data type for every steal outcome in a steal attempt because I have no idea how to handle this.
pub struct StealOutcomes {
  /// List of every player on a base
  pub players: Bases,
  /// List of every position on a base (kinda redundant but whatever)
  pub bases: Vec<usize>,
  /// The outcomes for each player's steal attempt
  pub steal_outcomes: Vec<Vec<StealOutcome>>,
}

impl StealOutcomes {
  pub fn new(base: Bases) -> StealOutcomes {
      let bases: Vec<usize> = base.iter().enumerate().map(|(i, x)| i).collect();
      let steal_outcomes = base.iter().map(|_| vec![]).collect();
      StealOutcomes {
          players: base,
          bases,
          steal_outcomes,
      }
  }

  /// Pushes a steal outcome to the respective base it belongs to.
  pub fn add_steal_outcome(&mut self, base: usize, outcome: StealOutcome) {
      let outcomes = &mut self.steal_outcomes[base];
      outcomes.push(outcome);
  }

  // Chops off any outcomes after a `CaughtStealing` outcome.
  pub fn chop(&mut self) {
      for outcome in self.steal_outcomes.iter_mut() {
          if outcome.len() != 0 {
              if outcome.contains(&StealOutcome::CaughtStealing) {
                  outcome.truncate(outcome.iter().position(|x| *x == StealOutcome::CaughtStealing).unwrap());
              }
          }
      }
  }

  pub fn iter(&self) -> StealOutcomesIter {
      StealOutcomesIter::new(self)
  }
}

impl IntoIterator for StealOutcomes {
  type Item = (Option<PlayerId>, Vec<StealOutcome>);
  type IntoIter = StealOutcomesIntoIter;

  fn into_iter(self) -> Self::IntoIter {
      StealOutcomesIntoIter {
        steal_outcomes: self,
        index: 0,
      }
  }
}

impl<'a> IntoIterator for &'a StealOutcomes {
  type Item = (Option<PlayerId>, Vec<StealOutcome>);
  type IntoIter = StealOutcomesIter<'a>;	

  fn into_iter(self) -> Self::IntoIter {
      StealOutcomesIter {
        steal_outcomes: self,
        index: 0,
      }
  }
}

pub struct StealOutcomesIter<'a> {
  steal_outcomes: &'a StealOutcomes,
	// the index
  index: usize,
}
impl <'a> StealOutcomesIter<'a> {
  fn new(steal_outcomes: &'a StealOutcomes) -> StealOutcomesIter<'a> {
      StealOutcomesIter {
          steal_outcomes,
          index: 0,
      }
  }
}

impl <'a> Iterator for StealOutcomesIter<'a> {
  type Item = (Option<PlayerId>, Vec<StealOutcome>);

  fn next(&mut self) -> Option<Self::Item> {
      if self.index >= self.steal_outcomes.steal_outcomes.len() {
          None
      } else {
          let outcome = self.steal_outcomes.steal_outcomes[self.index].clone();
          let player = self.steal_outcomes.players[self.index];
          self.index += 1;
          Some((player, outcome))
      }
  }
}


pub struct StealOutcomesIntoIter {
  steal_outcomes: StealOutcomes,
  index: usize,
}

impl Iterator for StealOutcomesIntoIter {
  type Item = (Option<PlayerId>, Vec<StealOutcome>);

  fn next(&mut self) -> Option<Self::Item> {
      if self.index >= self.steal_outcomes.steal_outcomes.len() {
          None
      } else {
          let outcome = self.steal_outcomes.steal_outcomes[self.index].clone();
          let player = self.steal_outcomes.players[self.index];
          self.index += 1;
          Some((player, outcome))
      }
  }
}