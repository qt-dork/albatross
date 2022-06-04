
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

  pub fn value(&self) -> f64 {
    if (self.is_dirty) {
      self._value = calculate_final_value();
      self.is_dirty = false;
    }
    self._value
  }

  pub fn add_modifier(&mut self, modifier: StatModifier) {
    self.is_dirty = true;
    self.stat_modifiers.push(modifier);
  }

  pub fn remove_modifier(&mut self, modifier: StatModifier) {
    self.is_dirty = true;
    self.stat_modifiers.retain(|m| m != modifier);
  }

  pub fn calculate_final_value(&self) -> f64 {
    let mut final_value = self.base_value;
    for modifier in &self.stat_modifiers {
      match modifier.get_type() {
        StatModifierType::Flat(value) => {
          final_value += value;
        }
        StatModifierType::Percent(value) => {
          final_value *= value;
        }
      }
    }
    final_value
  }
}

enum StatModifierType {
  Flat(f64),
  Percent(f64),
}
impl StatModifierType {
  pub fn get_value(&self) -> f64 {
    match self {
      StatModifier::Flat(value) => *value,
      StatModifier::Percent(value) => *value
    }
  }
}

#[derive(Debug, Clone)]
pub struct StatModifier {
  value: StatModifierType(f64),
}

impl StatModifier {
  pub fn new(value: f64, kind: StatModifierType) -> Self {
    StatModifier {
      value: StatModifierType(value),
    }
  }

  pub fn get_value(&self) -> f64 {
    self.value.get_value()
  }

  pub fn get_type(&mut self) -> StatModifierType {
    self.value.clone()
  }
}