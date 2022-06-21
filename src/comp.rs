// This is heavily based off of the Comp struct from:
// https://github.com/nsmryan/RustRoguelike/blob/4c434cd04389eebe2522638857b839d86ba80c81/roguelike_utils/src/comp.rs
// I could've just made a struct for everything and handled it that way but this is more cursed

use std::ops::{Index, IndexMut};

pub type EntityId = u64;
pub type PlayerId = EntityId;
pub type TeamId = EntityId;
pub type GameId = EntityId;

#[derive(Clone, Debug, PartialEq)]
pub struct Comp<T> {
    pub ids: Vec<EntityId>,
    pub store: Vec<T>,
}

impl<T> Comp<T> {
    pub fn new() -> Self {
        Comp {
            ids: Vec::new(),
            store: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity_id: EntityId, data: T) {
        let result = self.ids.binary_search(&entity_id);
        match result {
            Ok(ok_index) => {
                self.store[ok_index] = data;
            }
            Err(err_index) => {
                self.ids.insert(err_index, entity_id);
                self.store.insert(err_index, data);
            }
        }
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.store.clear();
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        let result = self.ids.binary_search(&entity_id);
        match result {
            Ok(index) => {
                self.ids.remove(index);
                self.store.remove(index);
            }
            Err(_) => {
                // do nothing
            }
        }
    }

    pub fn lookup(&self, entity_id: EntityId) -> usize {
        if let Ok(index) = self.ids.binary_search(&entity_id) {
            index
        } else {
            panic!("Could not find entity_id: {}", entity_id);
        }
    }

    pub fn get(&self, entity_id: &EntityId) -> Option<&T> {
        let result = self.ids.binary_search(entity_id);
        match result {
            Ok(index) => Some(&self.store[index]),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut T> {
        let result = self.ids.binary_search(entity_id);
        match result {
            Ok(index) => Some(&mut self.store[index]),
            Err(_) => None,
        }
    }

    pub fn contains_key(&self, entity_id: &EntityId) -> bool {
        self.ids.binary_search(entity_id).is_ok()
    }

    pub fn iter(&self) -> CompIter<'_, T> {
        return CompIter::new(self);
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }
}

impl<T> Default for Comp<T> {
    fn default() -> Self {
        Comp::new()
    }
}

impl<T> Index<&EntityId> for Comp<T> {
    type Output = T;

    fn index(&self, index: &EntityId) -> &T {
        let store_index = self.lookup(*index);
        &self.store[store_index]
    }
}

impl<T> IndexMut<&EntityId> for Comp<T> {
    fn index_mut(&mut self, index: &EntityId) -> &mut T {
        let store_index = self.lookup(*index);
        &mut self.store[store_index]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompIter<'a, T> {
    comp: &'a Comp<T>,
    index: usize,
}

impl <'a, T> CompIter<'a, T> {
    pub fn new(comp: &'a Comp<T>) -> Self {
        CompIter {
            comp,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for CompIter<'a, T> {
    type Item = (u64, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.comp.ids.len() {
            return None;
        } else {
            let index = self.index;
            self.index += 1;
            Some((self.comp.ids[index], &self.comp.store[index]))
        }
    }
}