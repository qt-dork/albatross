// use crate::java_random::Random;
// use crate::name_generator::Generator;
// use crate::player_stats::{CharacterStat, StatModifier, StatModifierType};
// use crate::player_flavor::{BloodType, CoffeeStyle, Fate, PregameRitual, Soulscream};

// #[derive(Debug, Clone)]
// pub struct Player {
//   pub name: String,
  
//   // ILLEGAL KNOWLEDGE; for devs only
//   // batting; ability to avoid flyouts
//   pub aggression: CharacterStat,
//   //baserunning; increases likelihood to attempt base-steal
//   pub arrogance: CharacterStat,
//   //defense; ability to prevent additional bases being runs
//   pub carcinization: CharacterStat,
//   //defense; ability to ground out batters
//   pub damage: CharacterStat,
//   //batting; higher density = smaller strike zone
//   pub density: CharacterStat,
//   //baserunning; how good a player is at stealing bases
//   pub dexterity: CharacterStat,
//   //pitching; makes batters more likely to strike, swinging
//   pub dimensions: CharacterStat,
//   //baserunning; ability to run additional bases
//   pub effort: CharacterStat,
//   //batting; increases likelihood to swing, avoid strike, looking.
//   pub focus: CharacterStat,
//   //pitching; decreases chance for batter to predict the pitch
//   pub fun: CharacterStat,
//   //pitching; makes batters more likely to strike, swinging.
//   pub grit: CharacterStat,
//   //baserunning; ability to avoid ground outs
//   pub hit_points: CharacterStat,
//   //batting; decreases chance to strike, swinging
//   pub malleability: CharacterStat,
//   //defense; increases chance to catch a flyout
//   pub mathematics: CharacterStat,
//   //batting; increases chance to predict the pitcher's throw.
//   pub number_of_eyes: CharacterStat,
//   //pitching; increases accuracy, lower chance to miss strike zone
//   pub pinpointedness: CharacterStat,
//   //pitching; increases chance for batter to hit a foul ball
//   pub powder: CharacterStat,
//   //defense; makes players less likely to attempt base-steals
//   pub rejection: CharacterStat,
//   //batting; decreases chance of hitting fouls when hitting the ball.
//   pub splash: CharacterStat,
//   //defense; decreases chance of successful base steals
//   pub wisdom: CharacterStat,

//   ritual: PregameRitual,
//   coffee: CoffeeStyle,
//   blood_type: BloodType,
//   fate: Fate,
//   soulscream: Soulscream,

//   // statistics: Statistics, // TODO: Make a statistics struct

//   pub id: u32,
// }
// impl Player {
//   pub fn new(rng: &mut Random, name: String) -> Player {
//     Player {
//       name: name.clone(),

//       aggression: Player::random_unweighted_stat(rng),
//       arrogance: Player::random_unweighted_stat(rng),
//       carcinization: Player::random_unweighted_stat(rng),
//       damage: Player::random_unweighted_stat(rng),
//       density: Player::random_unweighted_stat(rng),
//       dexterity: Player::random_unweighted_stat(rng),
//       dimensions: Player::random_unweighted_stat(rng),
//       effort: Player::random_unweighted_stat(rng),
//       focus: Player::random_unweighted_stat(rng),
//       fun: Player::random_unweighted_stat(rng),
//       grit: Player::random_unweighted_stat(rng),
//       hit_points: Player::random_unweighted_stat(rng),
//       malleability: Player::random_unweighted_stat(rng),
//       mathematics: Player::random_unweighted_stat(rng),
//       number_of_eyes: Player::random_unweighted_stat(rng),
//       pinpointedness: Player::random_unweighted_stat(rng),
//       powder: Player::random_unweighted_stat(rng),
//       rejection: Player::random_unweighted_stat(rng),
//       splash: Player::random_unweighted_stat(rng),
//       wisdom: Player::random_unweighted_stat(rng),

//       ritual: PregameRitual::get_random_ritual(rng),
//       coffee: CoffeeStyle::get_random_coffee_style(rng),
//       blood_type: BloodType::get_random_blood_type(rng),
//       fate: Fate::get_random_fate(rng),
//       soulscream: Soulscream::generate_soulscream(name),
      
//       // statistics: Statistics::new(),

//       id: rng.next_u32(),
//     }
//   }

//   pub fn default(rng: &mut Random) -> Player {
//     let mut gen = Generator::new(rng.get_seed());
//     let name = gen.next_name_with_distribution() + " " + &gen.next_name_with_distribution();
//     rng.set_seed(gen.get_seed());
//     Player::new(rng, name)
//   }

//   pub fn random_stat(rng: &mut Random) -> CharacterStat {
//     CharacterStat::new(rng.next_f64() * 2.5 + rng.next_f64() * 2.5)
//   }

//   pub fn random_unweighted_stat(rng: &mut Random) -> CharacterStat {
//     CharacterStat::new(rng.next_f64() * 5.0)
//   }

//   pub fn get_name(&self) -> String {
//     self.name.clone()
//   }

//   pub fn get_batting(&self) -> f64 {
//     (self.density.value() + self.number_of_eyes.value() / 2.0 + self.focus.value() / 4.0 + self.malleability.value() / 8.0 + self.splash.value() * 3.0 / 4.0 + self.aggression.value() * 3.0 / 32.0) / 2.28125
//   }

//   pub fn get_pitching(&self) -> f64 {
//     (self.pinpointedness.value() + self.fun.value() / 2.0 + self.grit.value() / 4.0 + self.dimensions.value() / 8.0 + self.powder.value() * 3.0 / 16.0 ) / 2.1875
//   }

//   pub fn get_baserunning(&self) -> f64 {
//     (self.hit_points.value() + self.effort.value() / 2.0 + self.arrogance.value() / 20.0 + self.dexterity.value() / 40.0) / 1.575
//   }

//   pub fn get_defense(&self) -> f64 {
//     (self.mathematics.value() + self.damage.value() / 2.0 + self.carcinization.value() / 4.0 + self.rejection.value() / 20.0 + self.wisdom.value() / 40.0) / 1.825
//   }

//   pub fn clear_temporary_modifiers(&mut self) {
//     self.aggression.clear_temporary_modifiers();
//     self.arrogance.clear_temporary_modifiers();
//     self.carcinization.clear_temporary_modifiers();
//     self.damage.clear_temporary_modifiers();
//     self.density.clear_temporary_modifiers();
//     self.dexterity.clear_temporary_modifiers();
//     self.dimensions.clear_temporary_modifiers();
//     self.effort.clear_temporary_modifiers();
//     self.focus.clear_temporary_modifiers();
//     self.fun.clear_temporary_modifiers();
//     self.grit.clear_temporary_modifiers();
//     self.hit_points.clear_temporary_modifiers();
//     self.malleability.clear_temporary_modifiers();
//     self.mathematics.clear_temporary_modifiers();
//     self.number_of_eyes.clear_temporary_modifiers();
//     self.pinpointedness.clear_temporary_modifiers();
//     self.powder.clear_temporary_modifiers();
//     self.rejection.clear_temporary_modifiers();
//     self.splash.clear_temporary_modifiers();
//     self.wisdom.clear_temporary_modifiers();
//   }

//   // I really don't like that it boosts them all by the same amount. I'd prefer to boost them with a modifier to make stronger stats get boosted less than weaker stats, or to boost individual stats.
//   pub fn boost_batting_by(&mut self, amount: f64) {
//     let modifier = StatModifier::new_without_longetivity(amount, StatModifierType::Flat);
//     self.aggression.add_modifier(modifier);
//     self.density.add_modifier(modifier);
//     self.focus.add_modifier(modifier);
//     self.malleability.add_modifier(modifier);
//     self.number_of_eyes.add_modifier(modifier);
//     self.splash.add_modifier(modifier);
//   }

//   pub fn boost_pitching_by(&mut self, amount: f64) {
//     let modifier = StatModifier::new_without_longetivity(amount, StatModifierType::Flat);
//     self.dimensions.add_modifier(modifier);
//     self.fun.add_modifier(modifier);
//     self.grit.add_modifier(modifier);
//     self.pinpointedness.add_modifier(modifier);
//     self.powder.add_modifier(modifier);
//   }

//   pub fn boost_baserunning_by(&mut self, amount: f64) {
//     let modifier = StatModifier::new_without_longetivity(amount, StatModifierType::Flat);
//     self.arrogance.add_modifier(modifier);
//     self.dexterity.add_modifier(modifier);
//     self.effort.add_modifier(modifier);
//     self.hit_points.add_modifier(modifier);
//   }

//   pub fn boost_defense_by(&mut self, amount: f64) {
//     let modifier = StatModifier::new_without_longetivity(amount, StatModifierType::Flat);
//     self.carcinization.add_modifier(modifier);
//     self.damage.add_modifier(modifier);
//     self.mathematics.add_modifier(modifier);
//     self.rejection.add_modifier(modifier);
//     self.wisdom.add_modifier(modifier);
//   }

//   // Stuff for messages
//   // pub fn bat_message(&self) -> String {}
//   // pub fn walk_message(&self) -> String {}

//   // Stuff for statistics
//   // pub fn clear_statistics(&mut self) {}
//   // pub fn add_statistic(&mut self, statistic: Statistic) {}
//   // pub fn add_statistic_with_value(&mut self, statistic: Statistic, value: f64) {}
//   // pub fn print_statistics(&self) {}
//   // pub fn get_statistics(&self) -> Vec<Statistic> {}

//   // Stuff for modifiers
//   // pub fn add_modifier(&mut self, modifier: Modifier) {}
//   // pub fn has_modifier(&self, modifier: Modifier) -> bool {}
//   // pub fn remove_modifier(&mut self, modifier: Modifier) {}
// }

// impl PartialEq for Player {
//   fn eq(&self, other: &Player) -> bool {
//     self.id == other.id
//   }
// }

// impl Eq for Player {}