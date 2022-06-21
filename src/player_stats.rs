use std::{ops::Add};


#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlayerStat {
  pub base_value: f64,
  stat_modifiers: Vec<StatModifier>,
}

impl PlayerStat {
  pub fn new(base_value: f64) -> Self {
    PlayerStat {
      base_value,
      stat_modifiers: Vec::new(),
    }
  }

  pub fn get_stat_modifiers(&self) -> Vec<StatModifier> {
    self.stat_modifiers.clone()
  }

  pub fn value(&self) -> f64 {
    self.calculate_final_value()
  }

  pub fn add_modifier(&mut self, modifier: StatModifier) {
    self.stat_modifiers.push(modifier);
    self.stat_modifiers.sort_by(|a, b| a.order.cmp(&b.order));
  }

  pub fn remove_modifier(&mut self, modifier: StatModifier) {
    self.stat_modifiers.retain(|m| m != &modifier);
  }

  pub fn clear_temporary_modifiers(&mut self) {
    self.stat_modifiers.retain(|m| !m.is_temporary());
  }
  
  pub fn clear(&mut self) {
    self.stat_modifiers.clear();
  }


  // I think i can do something with fold here idk
  // Also move this to value
  fn calculate_final_value(&self) -> f64 {
    let mut final_value = self.base_value;
    for modifier in &self.stat_modifiers {
      match modifier.stat_modifier_type {
        StatModifierType::Flat => {
          final_value += modifier.value;
        }
        StatModifierType::Percent => {
          final_value *= 1.0 + modifier.value;
        }
      }
    }
    final_value
  }

  pub fn base_value(&self) -> f64 {
    self.base_value
  }
}

// TODO: Implement other impls for CharacterStat, like Sub, Mul, etc.
impl Add for PlayerStat {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    let base_value = self.base_value + other.base_value;
    let stat_modifiers = self.get_stat_modifiers().into_iter().chain(other.get_stat_modifiers().into_iter()).collect();

    PlayerStat {
      base_value,
      stat_modifiers,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatModifierType {
  Flat,
  Percent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatModifierLongetivity {
  Permanent,
  Temporary,
}

#[derive(Debug, Clone, Copy)]
pub struct StatModifier {
  value: f64,
  stat_modifier_type: StatModifierType,
  stat_modifier_longetivity: StatModifierLongetivity,
  order: i32,
}

impl StatModifier {
  pub fn new(value: f64, stat_modifier_type: StatModifierType, stat_modifier_longetivity: StatModifierLongetivity, order: i32) -> Self {
    StatModifier {
      value,
      stat_modifier_type,
      stat_modifier_longetivity,
      order,
    }
  }

  pub fn new_without_order(value: f64, stat_modifier_type: StatModifierType, stat_modifier_longetivity: StatModifierLongetivity) -> Self {
    let order = stat_modifier_type.clone() as i32;
    StatModifier::new(value, stat_modifier_type, stat_modifier_longetivity, order)
  }

  pub fn new_without_longetivity(value: f64, stat_modifier_type: StatModifierType) -> Self {
    StatModifier::new_without_order(value, stat_modifier_type, StatModifierLongetivity::Temporary)
  }

  pub fn is_temporary(&self) -> bool {
    self.stat_modifier_longetivity == StatModifierLongetivity::Temporary
  }

  pub fn get_value(&self) -> f64 {
    self.value
  }

  pub fn get_type(&mut self) -> StatModifierType {
    self.stat_modifier_type.clone()
  }

  pub fn get_longetivity(&mut self) -> StatModifierLongetivity {
    self.stat_modifier_longetivity.clone()
  }

  pub fn get_order(&mut self) -> i32 {
    self.order
  }
}

impl Default for StatModifier {
  fn default() -> Self {
    let order = StatModifierType::Flat as i32;
    StatModifier {
      value: 0.0,
      stat_modifier_type: StatModifierType::Flat,
      stat_modifier_longetivity: StatModifierLongetivity::Temporary,
      order,
    }
  }
}

impl PartialEq for StatModifier {
  fn eq(&self, other: &Self) -> bool {
    self.stat_modifier_type == other.stat_modifier_type && self.value == other.value
  }
}

impl Eq for StatModifier {}