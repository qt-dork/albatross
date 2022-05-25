// So the goal of this file is to do all team- and player-related JSON parsing.
//use json_library; // for when we decide to import one
pub struct Team {

}

pub struct Player {
  name: String,
  batting: f64,
  pitching: f64,
  baserunning: f64,
  defense: f64,

  aggression: f64,
  anti_blasedness: f64,
  arrogance: f64,
  damage: f64,
  density: f64,
  dexterity: f64,
  dimensions: f64,
  effort: f64,
  focus: f64,
  fun: f64,
  grit: f64,
  hit_points: f64,
  malleability: f64,
  mathematics: f64,
  number_of_eyes: f64,
  pinpointedness: f64,
  powder: f64,
  rejection: f64,
  splash: f64,
  wisdom: f64,

  coffee: CoffeeStyle,
  blood_type: BloodType,
}

enum CoffeeStyle {
  Latte,
  ColdBrew,
  CreamAndSugar,
  Blood,
  PlentyOfSugar,
  Decaf,
  FlatWhite,
  Macchiato,
  MilkSubstitute,
  LightAndSweet,
  Americano,
  Espresso,
  HeavyFoam,
  CoffeeQuestion,
  Black,
  Anything,
  Tea,
}

impl CoffeeStyle {
  fn as_str(&self) -> &'static str {
    match self {
      CoffeeStyle::Latte => "Latte",
      CoffeeStyle::ColdBrew => "Cold Brew",
      CoffeeStyle::CreamAndSugar => "Cream & Sugar",
      CoffeeStyle::Blood => "Blood",
      CoffeeStyle::PlentyOfSugar => "Plenty of Sugar",
      CoffeeStyle::Decaf => "Decaf",
      CoffeeStyle::FlatWhite => "Flat White",
      CoffeeStyle::Macchiato => "Macchiato",
      CoffeeStyle::MilkSubstitute => "Milk Substitute",
      CoffeeStyle::LightAndSweet => "Light & Sweet",
      CoffeeStyle::Americano => "Americano",
      CoffeeStyle::Espresso => "Espresso",
      CoffeeStyle::HeavyFoam => "Heavy Foam",
      CoffeeStyle::CoffeeQuestion => "Coffee?",
      CoffeeStyle::Black => "Black",
      CoffeeStyle::Anything => "Anything",
      CoffeeStyle::Tea => "Tea",
    }
  }
}

enum BloodType {
  A,
  Aa,
  Aaa,
  Aaaaaaaaaa,
  Acidic,
  Basic,
  Electric,
  Fire,
  Grass,
  H2O,
  Love,
  O,
  ONo,
  Psychic,
  Dirt,
  B,
  Ab,
  Coffee,
  BloodQuestion,
}

impl BloodType {
  fn as_str(&self) -> &'static str {
    match self {
      BloodType::A => "A",
      BloodType::Aa => "AA",
      BloodType::Aaa => "AAA",
      BloodType::Aaaaaaaaaa => "AAAAAAAAAA",
      BloodType::Acidic => "Acidic",
      BloodType::Basic => "Basic",
      BloodType::Electric => "Electric",
      BloodType::Fire => "Fire",
      BloodType::Grass => "Grass",
      BloodType::H2O => "H2O",
      BloodType::Love => "Love",
      BloodType::O => "O",
      BloodType::ONo => "O No",
      BloodType::Psychic => "Psychic",
      BloodType::Dirt => "Dirt",
      BloodType::B => "B",
      BloodType::Ab => "AB",
      BloodType::Coffee => "Coffee",
      BloodType::BloodQuestion => "Blood?",
    }
  }
}