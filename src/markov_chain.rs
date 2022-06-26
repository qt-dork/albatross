use std::collections::HashMap;

use crate::java_random::Random;

//todo: make random name generator markov chain

struct MarkovChain {
    dictionary: HashMap<String, Vec<String>>,
    rng: Random,
}

impl MarkovChain {
    pub fn new() -> Self {
        MarkovChain {
            dictionary: HashMap::new(),
            rng: Random::new(0),
        }
    }
}