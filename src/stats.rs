use std::cmp::Ordering;


#[derive(Debug, Clone)]
pub struct CharacterStat {
  pub base_value: f64,
  stat_modifiers: Vec<StatModifier>,
  is_dirty: bool,
  _value: f64,
}

impl CharacterStat {
  pub fn new(base_value: f64) -> Self {
    CharacterStat {
      base_value,
      stat_modifiers: Vec::new(),
      is_dirty: false,
      _value: base_value,
    }
  }

  pub fn get_stat_modifiers(&self) -> Vec<StatModifier> {
    self.stat_modifiers.clone()
  }

  pub fn value(&mut self) -> f64 {
    if self.is_dirty {
      self._value = self.calculate_final_value();
      self.is_dirty = false;
    }
    self._value
  }

  pub fn add_modifier(&mut self, modifier: StatModifier) {
    self.is_dirty = true;
    self.stat_modifiers.push(modifier);
    self.stat_modifiers.sort_by(|a, b| a.order.cmp(&b.order));
  }

  pub fn remove_modifier(&mut self, modifier: StatModifier) {
    self.is_dirty = true;
    self.stat_modifiers.retain(|m| m != &modifier);
  }

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatModifierType {
  Flat,
  Percent,
}

#[derive(Debug, Clone, Copy)]
pub struct StatModifier {
  value: f64,
  stat_modifier_type: StatModifierType,
  order: i32,
}

impl StatModifier {
  pub fn new(value: f64, stat_modifier_type: StatModifierType, order: i32) -> Self {
    StatModifier {
      value,
      stat_modifier_type,
      order,
    }
  }

  pub fn new_without_order(value: f64, stat_modifier_type: StatModifierType) -> Self {
    let order = stat_modifier_type.clone() as i32;
    StatModifier::new(value, stat_modifier_type, order)
  }

  pub fn get_value(&self) -> f64 {
    self.value
  }

  pub fn get_type(&mut self) -> StatModifierType {
    self.stat_modifier_type.clone()
  }

  pub fn get_order(&mut self) -> i32 {
    self.order
  }
}

impl PartialEq for StatModifier {
  fn eq(&self, other: &Self) -> bool {
    self.stat_modifier_type == other.stat_modifier_type && self.value == other.value
  }
}

impl Eq for StatModifier {}