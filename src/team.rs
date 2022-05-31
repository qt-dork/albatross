// So the goal of this file is to do all team- and player-related JSON parsing.
//use json_library; // for when we decide to import one

use crate::java_random::Random;

pub struct Team {

}


#[derive(Debug)]
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

  ritual: PregameRitual,
  coffee: CoffeeStyle,
  blood_type: BloodType,
  fate: Fate,
  soulscream: Soulscream,
}
impl Player {
  pub fn new() -> Player {
    let mut rng = Random::new(0);
    let name = "Jerry".to_string();
    Player {
      name: name.clone(),
      batting: 0.0,
      pitching: 0.0,
      baserunning: 0.0,
      defense: 0.0,

      aggression: 0.0,
      anti_blasedness: 0.0,
      arrogance: 0.0,
      damage: 0.0,
      density: 0.0,
      dexterity: 0.0,
      dimensions: 0.0,
      effort: 0.0,
      focus: 0.0,
      fun: 0.0,
      grit: 0.0,
      hit_points: 0.0,
      malleability: 0.0,
      mathematics: 0.0,
      number_of_eyes: 0.0,
      pinpointedness: 0.0,
      powder: 0.0,
      rejection: 0.0,
      splash: 0.0,
      wisdom: 0.0,

      ritual: PregameRitual::get_random_ritual(&mut rng),
      coffee: CoffeeStyle::get_random_coffee_style(&mut rng),
      blood_type: BloodType::get_random_blood_type(&mut rng),
      fate: Fate::get_random_fate(&mut rng),
      soulscream: Soulscream::generate_soulscream(name),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum CoffeeStyle {
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
const COFFEE_VARIANTS: &[CoffeeStyle] = &[
  CoffeeStyle::Latte,
  CoffeeStyle::ColdBrew,
  CoffeeStyle::CreamAndSugar,
  CoffeeStyle::Blood,
  CoffeeStyle::PlentyOfSugar,
  CoffeeStyle::Decaf,
  CoffeeStyle::FlatWhite,
  CoffeeStyle::Macchiato,
  CoffeeStyle::MilkSubstitute,
  CoffeeStyle::LightAndSweet,
  CoffeeStyle::Americano,
  CoffeeStyle::Espresso,
  CoffeeStyle::HeavyFoam,
  CoffeeStyle::CoffeeQuestion,
  CoffeeStyle::Black,
  CoffeeStyle::Anything,
  CoffeeStyle::Tea,
];
impl CoffeeStyle {
  pub fn as_str(&self) -> &'static str {
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

  pub fn get_random_coffee_style(rng: &mut Random) -> CoffeeStyle {
    let x = rng.next_f64() * COFFEE_VARIANTS.len() as f64;
    COFFEE_VARIANTS[x as usize]
  }
}

#[derive(Debug, Clone, Copy)]
pub enum BloodType {
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
const BLOOD_TYPES: &[BloodType] = &[
  BloodType::A,
  BloodType::Aa,
  BloodType::Aaa,
  BloodType::Aaaaaaaaaa,
  BloodType::Acidic,
  BloodType::Basic,
  BloodType::Electric,
  BloodType::Fire,
  BloodType::Grass,
  BloodType::H2O,
  BloodType::Love,
  BloodType::O,
  BloodType::ONo,
  BloodType::Psychic,
  BloodType::Dirt,
  BloodType::B,
  BloodType::Ab,
  BloodType::Coffee,
  BloodType::BloodQuestion,
];
impl BloodType {
  pub fn as_str(&self) -> &'static str {
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

  pub fn get_random_blood_type(rng: &mut Random) -> BloodType {
    let x = rng.next_f64() * BLOOD_TYPES.len() as f64;
    BLOOD_TYPES[x as usize]
  }
}

const SOULSCREAM_CHARS: &'static str = "AEIOUHX";

#[derive(Debug)]
pub struct Soulscream(pub String);
impl Soulscream {
  pub fn generate_soulscream(name: String) -> Soulscream {
    let mut seed = 1;
    let mut soulscream = String::new();
    for i in 0..name.len() {
      seed = seed * name.chars().nth(i).unwrap() as i64;
    }
    let mut rng = Random::new(seed);
    let y = 1.0;
  
    for _ in 0..8 {
      let x = rng.next_f64() * SOULSCREAM_CHARS.len() as f64;
      soulscream.push(SOULSCREAM_CHARS.chars().nth(x as usize).unwrap());
    }
    loop {
      let y = y * 0.9;
      for _ in 0..8 {
        let x = rng.next_f64() * SOULSCREAM_CHARS.len() as f64;
        soulscream.push(SOULSCREAM_CHARS.chars().nth(x as usize).unwrap());
      }
      if rng.next_f64() < y {
        break;
      }
    }
    
    Soulscream(soulscream)
  }
  fn new_from_str(soulscream: String) -> Soulscream {
    Soulscream(soulscream)
  }
  fn as_str(&self) -> &str {
    let Soulscream(scream) = self;
    scream.as_str()
  }
}

const PREGAME_RITUALS: [&'static str; 343] = [
  "Trying their best",
  "Eating",
  "Sleeping",
  "Carcinization",
  "Squirming",
  "Looking for a way out",
  "Gonging",
  "Bees",
  "Shapeshifting",
  "Counting primes",
  "Doing nothing",
  "Listening",
  "Looking at the standings",
  "Trying to pronounce their own name",
  "Talking to god",
  "Deicide",
  "Tampering",
  "Singing",
  "Running the bases backwards",
  "Checking their pulse",
  "Eating a bird",
  "Eating multiple birds",
  "Saying hello",
  "Saying goodbye",
  "Having problems",
  "Forgetting",
  "Connecting",
  "Tying their shoes",
  "Losing their glove",
  "Brushing",
  "Staring",
  "Blinking",
  "Having an existential crisis",
  "Smiling",
  "Gambling",
  "Washing the blood off",
  "Coffee",
  "Hydration",
  "Winning",
  "Prepping",
  "Polishing",
  "Sit-ups",
  "Hello?",
  "Slithering",
  "Zooming",
  "Starting a trend",
  "Teleporting",
  "Existing",
  "Snack break!",
  "Colors",
  "Thinking about their secret crush",
  "Cloning",
  "Multitasking",
  "Particle Accelerator",
  "Looking",
  "Pitching",
  "Reading",
  "Heavy metal",
  "Tuning",
  "Lucid dreaming",
  "Begging",
  "Making friends",
  "Eating grass",
  "Reassembling",
  "Breadmaking",
  "Knitting",
  "Spitting",
  "Plenty of Sugar",
  "Preparing for the worst",
  "Hoping for the best",
  "Painting",
  "*crunch*",
  "Juggling",
  "Gaming",
  "Tax evasion",
  "Unionizing",
  "Feeding Carl",
  "Melting",
  "Blaseing",
  "Waltzing",
  "Spinning in circles",
  "Spinning in squares",
  "Soup night",
  "Hiding",
  "Seeking",
  "Research",
  "Reading lore",
  "Swearing",
  "Procrastinating",
  "Looking at their stats",
  "Explosion",
  "Spooning",
  "Screaming",
  "Ultraviolet lights",
  "Imaginary cow",
  "They know",
  "Swimming",
  "Cleaning their bat",
  "Laps",
  "Befriending the leeches",
  "Pretending everything's fine",
  "crab math",
  "Simulating",
  "Chasing down dreams",
  "Go fish",
  "Hopscotch",
  "Scotch",
  "Marine biology",
  "Rolling coins",
  "Crumping",
  "Skipping stones",
  "Fall Guys Among Us",
  "Getting away from it all",
  "Succulents",
  "Pizza",
  "Monobob",
  "Extreme ironing",
  "Cheese rolling",
  "Cheese grating",
  "Smelling old books",
  "Doing cartwheels",
  "Basket weaving",
  "Writing lore",
  "Consuming",
  "Smiting mortals",
  "Hugging",
  "Calling your manager",
  "Getting cozy",
  "Studying Pataphysics",
  "Discord",
  "Discourse",
  "Making cheesecake",
  "Necromancy",
  "Counting beans",
  "Crying",
  "Vibrating",
  "Reincarnation",
  "Vibing",
  "Troubleshooting",
  "Shaking their fist at a bird's nest",
  "Yelling at clouds",
  "Tooth collecting",
  "Setting the clock an hour forwards",
  "Up to interpretation",
  "Brass",
  "Interpreting",
  "Planking",
  "Checking the mail",
  "Ragtime",
  "Spider climbing",
  "Tickling",
  "Being the ball",
  "Picking up lungs",
  "Picking scabs",
  "Antiquing",
  "Petting",
  "Soapstone",
  "Resetting the Wi-Fi",
  "Ritual?",
  "Inventing new swear words",
  "Downvoting",
  "Upvoting",
  "Backreading",
  "Reforestation",
  "Defenestration",
  "Combo breaking",
  "Trolling",
  "Causing paradoxes",
  "\"Do I really need one?\"",
  "Online shopping",
  "Stealing hearts",
  "Returning hearts",
  "Gacha",
  "Breaking infinities",
  "Wlordle",
  "Gaslighting",
  "Gatekeeping",
  "Girlbossing",
  "Throwing punches",
  "Catching punches",
  "Blanking",
  "Fiber arts",
  "Tables",
  "Card games",
  "Bonking",
  "Blocking the sun",
  "Swabbing the deck",
  "Bridging",
  "Thinking of pregame rituals",
  "Escaping",
  "Keymashing",
  "Locking all the doors",
  "Neighing",
  "Searching",
  "Keeping it together",
  "Yes",
  "No",
  "Scratching their head",
  "Whisking",
  "Divining the stars",
  "Sighs",
  "Being sheriff",
  "Burning",
  "Blubbering",
  "Psychogeography",
  "Unwarranted arrogance",
  "Ingesting poison",
  "Theft",
  "Shadowboxing",
  "Philosophy",
  "Butchering",
  "Baking",
  "Leaving breadcrumbs",
  "Lava",
  "Birdwatching",
  "Oobleck",
  "Salad spinning",
  "Jellyfishing",
  "Cereal",
  "Parallel parking",
  "Coloring outside the lines",
  "Gun bong",
  "Contrarianism",
  "Being cool",
  "Self-inserting",
  "Holding hands",
  "Counting rings",
  "Yeeting",
  "tYpInG lIkE tHiS",
  "Digging for buried treasure",
  "Wishing they had played better yesterday",
  "Hiding from teammates",
  "Putting whoopie cushions under the bases",
  "Horseback riding",
  "Staring at their reflection",
  "photosynthesis",
  "Tearing up the daily newspaper",
  "Crosswords",
  "Plucking feathers",
  "Laughing uncontrollably",
  "cursing the gods",
  "Leaving ominous notes",
  "Running errands",
  "Pressing the big red button",
  "Calculating",
  "Mooing",
  "Boiling water",
  "Melting wood",
  "Letting it out",
  "Chaos",
  "Stealing shoes",
  "Magic",
  "Smashing a typewriter",
  "X-rays",
  "Microwaves",
  "ctrl+z",
  "rotating",
  "Merging timelines",
  "Forbidden knowledge",
  "Breakdancing",
  "AP Calculus",
  "Power nap",
  "Building a table",
  "Hardcore plumbing",
  "Freezing",
  "Punching the air",
  "Advanced Origami",
  "Advanced hopscotch",
  "Ignoring the voices",
  "Ignoring the voices",
  "Googling themself",
  "The cha cha slide",
  "Poetry",
  "Staring into the sun",
  "Twitch streaming",
  "Propaganda",
  "Fighting fires",
  "Drama",
  "Drinking the juice",
  "The void",
  "Divorce",
  "Gardening",
  "Default",
  "Responding to emails",
  "Denying accusations",
  "Spam mail",
  "Waiting for the game to start",
  "Pondering shrimp",
  "Standing up straight",
  "Perpetual motion",
  "Backflips",
  "Rocket science",
  "peeling fruit",
  "Admiring their collection",
  "Selling copper wire on craigslist",
  "Threatening umps",
  "Something evil",
  "Showing up late",
  "Violence",
  "Remembering before",
  "Feeding the floor monster",
  "Camouflage",
  "Phone games from 2009",
  "Dressing up",
  "Piracy",
  "Drinking battery acid",
  "Catching up",
  "Checking the forecast",
  "Feeding the birds",
  "Catching up",
  "Saving the world",
  "Holding their breath",
  "Changing color",
  "Evolving",
  "Flickering",
  "Mood swings",
  "Math homework",
  "Giving a monkey a shower",
  "Mandelbrot",
  "Surfing",
  "Painting the bases",
  "Partying",
  "Licking the moon",
  "Tasting the infinite",
  "Standup comedy",
  "Food crimes",
  "Overachieving",
  "Organ borrowing",
  "Theatrics",
  "Summoning a dragon",
  "Crime",
  "Breaking a mirror",
  "Dodge rolling",
  "OFFLINE",
  "Leaking",
  "Making slime",
  "Unknowing the known",
  "Weasels",
  "Vandalism",
  "Skateboard trick",
  "Reading the room",
  "Picking flowers",
  "Side jobs"
];
#[derive(Debug)]
pub struct PregameRitual(String);
impl PregameRitual {
  fn get_random_ritual(rng: &mut Random) -> PregameRitual {
    let x = rng.next_f64() * PREGAME_RITUALS.len() as f64;
    let ritual = PREGAME_RITUALS[x as usize];

    PregameRitual(ritual.to_string())
  }
  pub fn is_valid(&self) -> bool {
    let PregameRitual(ritual_string) = self;
    let ritual_string = ritual_string.clone();
    PREGAME_RITUALS.contains(&ritual_string.as_str())
  }
}

#[derive(Debug)]
pub struct Fate(u8);
impl Fate {
  pub fn get_random_fate(rng: &mut Random) -> Fate {
    let fate = rng.next_f64() * 100.0;
    Fate(fate as u8)
  }
  pub fn as_u8(&self) -> u8 {
    let Fate(value) = self;
    value.clone()
  }
  pub fn is_valid(&self) -> bool {
    self.as_u8() <= 100
  }
}