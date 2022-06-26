use crate::player_stat::*;
use crate::java_random::Random;

// There's bound to be other stuff but at the very least, this cleans stuff up
#[derive(Clone, Debug, PartialEq)]
pub struct Attributes {
    pub aggression: PlayerStat,
    pub arrogance: PlayerStat,
    pub carcinization: PlayerStat,
    pub damage: PlayerStat,
    pub density: PlayerStat,
    pub dexterity: PlayerStat,
    pub dimensions: PlayerStat,
    pub effort: PlayerStat,
    pub focus: PlayerStat,
    pub fun: PlayerStat,
    pub grit: PlayerStat,
    pub hit_points: PlayerStat,
    pub malleability: PlayerStat,
    pub mathematics: PlayerStat,
    pub number_of_eyes: PlayerStat,
    pub pinpointedness: PlayerStat,
    pub powder: PlayerStat,
    pub rejection: PlayerStat,
    pub splash: PlayerStat,
    pub wisdom: PlayerStat,
}

impl Attributes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(
        aggression: PlayerStat,
        arrogance: PlayerStat,
        carcinization: PlayerStat,
        damage: PlayerStat,
        density: PlayerStat,
        dexterity: PlayerStat,
        dimensions: PlayerStat,
        effort: PlayerStat,
        focus: PlayerStat,
        fun: PlayerStat,
        grit: PlayerStat,
        hit_points: PlayerStat,
        malleability: PlayerStat,
        mathematics: PlayerStat,
        number_of_eyes: PlayerStat,
        pinpointedness: PlayerStat,
        powder: PlayerStat,
        rejection: PlayerStat,
        splash: PlayerStat,
        wisdom: PlayerStat,
    ) -> Self {
        Attributes {
            aggression,
            arrogance,
            carcinization,
            damage,
            density,
            dexterity,
            dimensions,
            effort,
            focus,
            fun,
            grit,
            hit_points,
            malleability,
            mathematics,
            number_of_eyes,
            pinpointedness,
            powder,
            rejection,
            splash,
            wisdom,
        }
    }
    
    pub fn from_f64(
        aggression: f64,
        arrogance: f64,
        carcinization: f64,
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
    ) -> Self {
        Attributes {
            aggression: PlayerStat::new(aggression),
            arrogance: PlayerStat::new(arrogance),
            carcinization: PlayerStat::new(carcinization),
            damage: PlayerStat::new(damage),
            density: PlayerStat::new(density),
            dexterity: PlayerStat::new(dexterity),
            dimensions: PlayerStat::new(dimensions),
            effort: PlayerStat::new(effort),
            focus: PlayerStat::new(focus),
            fun: PlayerStat::new(fun),
            grit: PlayerStat::new(grit),
            hit_points: PlayerStat::new(hit_points),
            malleability: PlayerStat::new(malleability),
            mathematics: PlayerStat::new(mathematics),
            number_of_eyes: PlayerStat::new(number_of_eyes),
            pinpointedness: PlayerStat::new(pinpointedness),
            powder: PlayerStat::new(powder),
            rejection: PlayerStat::new(rejection),
            splash: PlayerStat::new(splash),
            wisdom: PlayerStat::new(wisdom),
        }
    }

    pub fn random(rng: &mut Random) -> Self {
        Attributes::from_f64(
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64(), 
            rng.next_f64()
        )
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            aggression: PlayerStat::default(),
            arrogance: PlayerStat::default(),
            carcinization: PlayerStat::default(),
            damage: PlayerStat::default(),
            density: PlayerStat::default(),
            dexterity: PlayerStat::default(),
            dimensions: PlayerStat::default(),
            effort: PlayerStat::default(),
            focus: PlayerStat::default(),
            fun: PlayerStat::default(),
            grit: PlayerStat::default(),
            hit_points: PlayerStat::default(),
            malleability: PlayerStat::default(),
            mathematics: PlayerStat::default(),
            number_of_eyes: PlayerStat::default(),
            pinpointedness: PlayerStat::default(),
            powder: PlayerStat::default(),
            rejection: PlayerStat::default(),
            splash: PlayerStat::default(),
            wisdom: PlayerStat::default(),
        }
    }
}