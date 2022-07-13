use std::ops::{Index, IndexMut};

use crate::comp::PlayerId;

#[derive(Clone, Debug, PartialEq)]
pub struct Bases {
  pub bases: Vec<Option<PlayerId>>,
	// probably unnecessary
  size: usize,
}

impl Bases {
  /// Creates an empty set of bases of a given size.
  pub fn new(size: usize) -> Bases {
    Bases {
      bases: vec![None; size],
      size,
    }
  }

  /// Creates a new set of bases from a vector of players.
  pub fn from(bases: &[Option<PlayerId>]) -> Bases {
		Bases {
			bases: bases.to_vec(),
			size: bases.len(),
		}
  }

  /// Checks if there are 4 bases (the fourth base is implied).
  /// 
  /// Returns true if there are 4 bases, false otherwise.
  /// 
  /// I guess there are situations where there could be less than four bases, which would also result in it returning true, although that hasn't been implemented at all.
  pub fn is_non_standard(&self) -> bool {
    self.size != 3
  }

  /// Resizes the bases to a new size.
  /// 
  /// If the new size is larger than the old size, then the new base will be empty, or filled with `None`.
  /// However, if the new size is smaller than the old size, then any players on that base will be returned, as they have scored.
  /// 
  /// This is confirmed to be how it works in Blaseball, as shown in [this Reblase event](https://reblase.sibr.dev/game/774ac25c-959a-489c-bd16-fd0a64811fa1#7b8616b4-586e-3dad-2187-bc45fd1384b9).
  pub fn resize(&mut self, size: usize) -> Option<PlayerId> {
		let mut removed: Option<u64> = None;

		// If a player is in the last base, and that base gets removed, then that player scores.
		if self.bases.len() > size {
				removed = self.bases.pop()?;
		}
		self.bases.resize(size, None);
		self.size = size;
		removed
  }

  /// Adds a new base.
  /// 
  /// This function is meant for situations like **The Fifth Base** being placed in the game, although it's currently not implemented in Albatross.
  pub fn add_base(&mut self) {
		self.bases.push(None);
		self.size += 1;
  }

  /// Removes a base. Returns the player on that base, if there is one.
  /// 
  /// This function is meant for situations like **The Fifth Base** being stolen from a game, although it's currently not implemented in Albatross.
  pub fn remove_base(&mut self) -> Option<PlayerId> {
    self.bases.pop()?
  }

  /// Gets a reference to a player from a base.
  /// 
  /// This function uses the standard vector `get()` function, so the functionality is equivalent to how get works in Rust.
  pub fn get(&self, base: usize) -> Option<&PlayerId> {
    self.bases.get(base)?.as_ref()
  }

  /// Gets a mutable reference to a player from a base. 
  /// 
  /// This uses Rust's standard vector `get_mut()` function, so it works the same.
  /// 
  /// # Examples
  /// ```rust
  /// let mut bases = Bases::from(&[Some(1), Some(2), Some(3), Some(4)]);
  /// if let Some(player) = bases.get_mut(2) {
  ///    *player = 5;
  /// }
  /// assert_eq!(bases.get(2), Some(&5));
  /// ```
  pub fn get_mut(&mut self, base: usize) -> Option<&mut PlayerId> {
    self.bases.get_mut(base)?.as_mut()
  }

  /// Inserts a player onto first base. If this results in a player reaching home, then that player is returned. Otherwise, [`None`] is returned.
  /// 
  /// This function is meant to be used when a player reaches the first base by any means, like by walk or by hit.
  pub fn push_front(&mut self, player: PlayerId) -> Option<PlayerId> {
    self.bases.insert(0, Some(player));
    self.bases.pop()?
  }

  pub fn insert(&mut self, base: usize, player: PlayerId) -> Option<PlayerId> {
    self.bases.insert(base, Some(player));
    self.bases.pop()?
  }

  /// Advances every player on a base by one base. If a player reaches home because of this then that player is returned. Otherwise, `None` is returned.
  pub fn advance(&mut self) -> Option<PlayerId> {
		self.bases.rotate_right(1);
		match self.bases[0] {
			Some(player_id) => {
				self.bases[0] = None;
				Some(player_id)
			}
			None => None,
		}
  }

  /// Advances a single player by one base. If the player reaches home, then that player is returned. Otherwise, [`None`] is returned.
  /// 
  /// This function is primarily meant to be used when a player successfully steals a base, as I think it's the only way to advance a single player.
  /// 
  /// This function returns early if you try to advance a base that isn't occupied. This is to avoid any situations where you interact with a player that doesn't exist.
  /// 
  /// # Panics
  /// This function will panic if you try to advance a player to a position that is already occupied.
  // changed from player to position because it's possible that the same player can occupy multiple bases
  pub fn advance_base(&mut self, position: usize) -> Option<PlayerId> {
    // Stores the player at the positions
    let player = self.bases[position].take();
    if player.is_none() {
      return None;
    }

    // Convoluted way to advance only the base the player is on by one.
    self.bases.rotate_left(1);
    if self.bases[position].is_none() {
      self.bases[position] = player;
    } else {
      panic!("Tried to advance a base that was already occupied!");
    }
    self.bases.rotate_right(1);

    // Check if the player has reached home.
    if self.bases[0].is_some() {
      self.bases[position].take()
    } else {
      None
    }
  }

  /// Returns `true` if a player is on a base.
  pub fn is_occupied(&self, position: usize) -> bool {
    self.bases[position].is_some()
  }

  /// Checks if the bases from a range of positions are occupied. Returns `true` if all of the bases are occupied.
  pub fn is_occupied_range(&self, start: usize, end: usize) -> bool {
    for i in start..end {
      if !self.is_occupied(i) {
        return false;
      }
    }
    true
  }

	pub fn is_next_occupied(&self, position: usize) -> bool {
		if position == self.size - 1 {
			return false;
		}
		self.is_occupied(position + 1)
	}

  pub fn contains(&self, player: PlayerId) -> Option<usize> {
    for (i, base) in self.bases.iter().enumerate() {
      if base.is_some() && base.unwrap() == player {
        return Some(i);
      }
    }
    None
  }

	pub fn is_empty(&self) -> bool {
		self.bases.iter().all(|base| base.is_none())
	}

	/// Returns the number of players on the bases.
	pub fn len(&self) -> usize {
		self.bases.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &Option<PlayerId>> {
		self.bases.iter()
	}
}

impl Default for Bases {
  fn default() -> Self {
    Bases {
      bases: vec![None; 3],
      size: 3,
    }
  }
}

impl Index<usize> for Bases {
  type Output = Option<PlayerId>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.bases[index]
  }
}

impl IndexMut<usize> for Bases {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.bases[index]
  }
}

// Github Copilot suggested all of this lol
impl From<&[Option<PlayerId>]> for Bases {
  fn from(bases: &[Option<PlayerId>]) -> Self {
    Bases {
      bases: bases.to_vec(),
      size: bases.len(),
    }
  }
}

impl From<&[PlayerId]> for Bases {
  fn from(bases: &[PlayerId]) -> Self {
    Bases {
      bases: bases.iter().map(|&player_id| Some(player_id)).collect(),
      size: bases.len(),
    }
  }
}