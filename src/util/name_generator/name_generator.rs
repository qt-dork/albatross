use crate::util::rng::Rand32;

use super::markov_chain::MarkovChain;

use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn read_names_from_file<P: AsRef<Path>>(path: P) -> Result<Names, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Names = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

#[derive(Debug, Deserialize)]
struct Names {
    names_first: Vec<String>,
    names_last: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NameGenerator {
    gen_first: MarkovChain,
    gen_last: MarkovChain,
}

impl NameGenerator {
    pub fn new() -> Self {
        NameGenerator {
            gen_first: MarkovChain::new(),
            gen_last: MarkovChain::new(),
        }
    }

    pub fn load_names(&mut self) -> Result<(), Box<dyn Error>> {
        let names = read_names_from_file("src/util/name_generator/names.json")?;
        for name in names.names_first {
            self.gen_first.add_to_dictionary(&name);
        }
        for name in names.names_last {
            self.gen_last.add_to_dictionary(&name);
        }
        Ok(())
    }

    pub fn generate_name(&mut self, rng: &mut Rand32) -> String {
        let first = self.gen_first.generate_name(rng);
        let last = self.gen_last.generate_name(rng);
        format!("{} {}", first, last)
    }
}