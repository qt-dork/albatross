
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Mods {
    pub mods: Vec<Modification>,
    pub mods_target: Vec<ModificationTarget>,
    pub mods_duration: Vec<ModificationDuration>,
}

impl Mods {
    pub fn new() -> Self {
        Mods {
            mods: Vec::new(),
            mods_target: Vec::new(),
            mods_duration: Vec::new(),
        }
    }

    pub fn insert(&mut self, modification: Modification) {
        let result = self.mods.contains(&modification);
        match result {
            true => {
                // do nothing
            }
            false => {
                self.mods.push(modification);
                self.mods_target.push(modification.get_modification_target());
                self.mods_duration.push(modification.get_modification_duration());
            }
        }
    }

    pub fn clear(&mut self) {
        self.mods.clear();
        self.mods_target.clear();
        self.mods_duration.clear();
    }

    pub fn remove(&mut self, modification: &Modification) {
        let result = self.mods.contains(modification);
        match result {
            true => {
                let index = self.mods.iter().position(|&x| x == *modification).unwrap();
                self.mods.remove(index);
                self.mods_target.remove(index);
                self.mods_duration.remove(index);
            }
            false => {
                // do nothing
            }
        }
    }

    pub fn lookup(&self, modification: &Modification) -> usize {
        if let Some(index) = self.mods.iter().position(|&x| x == *modification) {
            index
        } else {
            panic!("Could not find modification: {:?}", modification);
        }
    }

    pub fn has(&self, modification: &Modification) -> bool {
        self.mods.contains(modification)
    }

    pub fn get(&self, modification: &Modification) -> Option<&(Modification, ModificationTarget, ModificationDuration)> {
        let result = self.mods.iter().position(|&x| x == *modification);
        match result {
            Some(index) => {
                Some(&(self.mods[index], self.mods_target[index], self.mods_duration[index]))
            },
            None => None,
        }
    }
}

impl PartialEq for Mods {
    fn eq(&self, other: &Self) -> bool {
        self.mods == other.mods
    }
}

impl Eq for Mods {}

impl Default for Mods {
    fn default() -> Self {
        Mods::new()
    }
}

impl Index<usize> for Mods {
    type Output = (Modification, ModificationTarget, ModificationDuration);

    fn index(&self, index: usize) -> &Self::Output {
        &(self.mods[index], self.mods_target[index], self.mods_duration[index])
    }
}

impl IndexMut<usize> for Mods {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.mods[index], self.mods_target[index], self.mods_duration[index])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModsIter {
    mods: Mods,
    index: usize,
}

impl ModsIter {
    pub fn new(mods: Mods) -> Self {
        ModsIter {
            mods,
            index: 0,
        }
    }
}

impl Iterator for ModsIter {
    type Item = (Modification, ModificationTarget, ModificationDuration);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.mods.mods.len() {
            None
        } else {
            let index = self.index;
            self.index += 1;
            Some((self.mods.mods[index], self.mods.mods_target[index], self.mods.mods_duration[index]))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModificationTarget {
    Player,
    Team,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModificationDuration {
    Permanent,
    Season, // Not implemented yet
    Weekly, // Not implemented yet
    Game,

    // Item, // This is in blaseball but not in alb
    // Armor, // This is in blaseball but not in alb
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modification {
    Cat, // Remove
    Elsewhere,
}

impl Modification {
    pub fn get_name(&self) -> String {
        match self {
            Modification::Cat => "Cat".to_string(),
            Modification::Elsewhere => "Elsewhere".to_string(),
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            Modification::Cat => "Meow".to_string(),
            Modification::Elsewhere => "Elsewhere".to_string(),
        }
    }

    pub fn get_modification_target(&self) -> ModificationTarget {
        match self {
            Modification::Cat => ModificationTarget::Player,
            Modification::Elsewhere => ModificationTarget::Player,
        }
    }

    pub fn get_modification_duration(&self) -> ModificationDuration {
        match self {
            Modification::Cat => ModificationDuration::Permanent,
            Modification::Elsewhere => ModificationDuration::Permanent,
        }
    }
}