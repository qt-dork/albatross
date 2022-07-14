use crate::util::rng::Rand32;

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

const START_SYMBOL: &str = "^";
const END_SYMBOL: &str = "$";

#[derive(Debug, Clone)]
pub struct MarkovChain {
    pub chain: HashMap<String, Vec<String>>,

}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            chain: HashMap::new(),
        }
    }

    pub fn add_to_dictionary(&mut self, name: &str) {
        let name = format!("^^{}$$", name);
        for i in 0..name.graphemes(true).collect::<Vec<&str>>().len() - 3 {
            let combo = &name.graphemes(true).collect::<Vec<&str>>()[i..i + 2].join("");
            let append = self.chain.entry(combo.to_string()).or_insert(vec![]);
            let push = name.graphemes(true).nth(i + 2).unwrap();
            append.push(push.to_string());
        }
    }

    pub fn generate_name(&mut self, rng: &mut Rand32) -> String {
        let mut combo = "^^".to_string();
        let mut next_letter = "".to_string();
        let mut result = "".to_string();

        let mut length = 0;
        let mut countdown = 10;
        let mut final_countdown = 100;
        let mut too_long = false;

        loop {
            let number_of_letters = self.chain.entry(combo.clone()).or_default().len();
            // get random usize
            let index = rng.rand_usize() % number_of_letters;
            next_letter = self.chain.entry(combo.clone()).or_default()[index].clone();
            if length > 9 {
                if next_letter != END_SYMBOL && countdown > 0 {
                    countdown -= 1;
                    continue;
                }
            } else if length > 12 {
                if next_letter != END_SYMBOL && final_countdown > 0 {
                    final_countdown -= 1;
                    continue;
                } else if final_countdown == 0 && too_long == false {
                    too_long = true;
                } else if final_countdown == 0 && too_long == true {
                    break;
                }
            }
            countdown = 10;
            if next_letter == END_SYMBOL {
                break;
            } else {
                result.push_str(&next_letter);
                length += 1;
                combo = combo.graphemes(true).nth(1).unwrap().to_string() + &next_letter;
            }
        }

        return result;
    }
}