//! This library has been dropped in favor
//! of using `name_generator`, a name
//! generator which is based off of a
//! markov chain, instead of whatever the
//! hell this is.
//! 
//! Future versions of albatross will not
//! use this library, and it may be
//! removed completely upon release.

// I think it'd be nice to make this a markov chain at a later point in time, but for now it'll just be the regular name generator.

use crate::util::java_random::Random;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";
const VOWELS_MINUS_Y: &str = "aeiou";
const VOWELS: &str = "aeiouy";

const ENGLISH_LETTER_DISTRIBUTION: [f64; 26] = [8.5,2.7,4.54,3.38,11.16,1.81,2.47,3.0,7.54,0.2,1.1,5.49,3.01,6.65,7.16,3.17,0.2,7.58,5.74,6.95,3.63,1.01,1.29,0.29,1.78,0.27];
const ENGLISH_CONSONANT_DISTRIBUTION: [f64; 21] = [2.7,4.54,3.38,1.81,2.47,3.0,0.2,1.1,5.49,3.01,6.65,3.17,0.2,7.58,5.74,6.95,1.01,1.29,0.29,1.78,0.27];
const ENGLISH_VOWEL_DISTRIBUTION: [f64; 6] = [8.5,11.16,7.54,7.16,3.63,1.78];
const ENGLISH_VOWEL_DISTRIBUTION_MINUS_Y: [f64; 5] = [8.5,11.16,7.54,7.16,3.63];

enum ConstTypes {
    Letters,
    Consonants,
    Vowels,
    VowelsMinusY,
}

/// A random name generator.
pub struct MixedNameGenerator {
    rng: Random,
}

impl MixedNameGenerator {
    pub fn new(seed: i64) -> Self {
        MixedNameGenerator {
            rng: Random::new(seed),
        }
    }
    pub fn next_name(&mut self) -> String {
        let len = self.random(1, 5);
        let len = len + self.random(1, 5);
        let nth = LETTERS.chars().nth(self.random(1, LETTERS.len())).unwrap();
        self.generate_name(
            len, 
            nth.to_string()
        )
    }

    pub fn with_length(&mut self, length: usize) -> String {
        let nth = LETTERS.chars().nth(self.random(1, LETTERS.len() - 1)).unwrap();
        self.generate_name(length,nth.to_string())
    }

    pub fn with_starting_letter(&mut self, starting_letter: char) -> String {
        let length = self.random(1, 5) + self.random(1, 5);
        self.generate_name(
            length, 
            starting_letter.to_string()
        )
    }

    pub fn name_from(&mut self, starting_letters: String) -> String {
        let length = self.random(1, 5) + self.random(1, 5);
        self.generate_name(
            length, 
            starting_letters
        )
    }

    pub fn generate_name(&mut self, length: usize, starting_letters: String) -> String {
        let mut name = starting_letters.clone();
        let mut l = length;
        if starting_letters.len() == 0 {
            name = LETTERS.chars()
                .nth(self.random(
                    0, 
                    LETTERS.len() - 1)).unwrap()
                        .to_string();
        }

        while l <= name.len() {
            if name.len() >= 12 {
                l = name.len() + 1;
            } else {
                l = self.random(1, 6) + self.random(1, 6);
            }
        }

        let mut mode = self.get_mode(&name, l);
        let x = name.len();
        name = capitalize(&name);

        for i in x..l {
            let mut add = 'a';
            add = match mode {
                0 => {
                    let mut a = VOWELS.chars()
                        .nth(self.random(0, 4))
                        .unwrap();
                    if i == length - 1 {
                        a = VOWELS.chars().nth(self.random(0, 5)).unwrap();
                    }
                    mode += 1;

                    a
                }
                1 => {
                    let a = VOWELS.chars()
                        .nth(self.random(0, 4))
                        .unwrap();
                    mode = self.random(2, 3);

                    a
                }
                2 => {
                    let a = CONSONANTS.chars()
                        .nth(self.random(0, CONSONANTS.len() - 1))
                        .unwrap();
                    if i == length - 2 {
                        mode = 0
                    } else {
                        mode += 1;
                    }

                    a
                }
                3 => {
                    let a = CONSONANTS.chars()
                        .nth(self.random(0, CONSONANTS.len() - 1))
                        .unwrap();
                    mode = self.random(0, 1);

                    a
                }
                _ => {
                    'a'
                }
            };
            name.push(add);
        }
        name
    }

    pub fn next_name_with_distribution(&mut self) -> String {
        let len = self.random(1, 4) + self.random(2, 4);
        self.generate_name_with_distribution(len, "".to_string()
        )
    }

    pub fn distribution_with_length(&mut self, length: usize) -> String {
        self.generate_name_with_distribution(length, "".to_string())
    }

    pub fn generate_name_with_distribution(&mut self, length: usize, starting_letters: String) -> String {
        let mut n = starting_letters.clone();
        let mut l = length;
        if starting_letters.len() == 0 {
            n = self.get_random_letter(ConstTypes::Letters).to_string();
        }
        while l <= n.len() {
            if n.len() >= 12 {
                l = n.len() + 1;
            } else {
                l = self.random(1, 6) + self.random(1, 6);
            }
        }

        let mut mode = self.get_mode(&n, l);
        let mut name = n.clone();
        for i in n.len()..l {
            let mut add = 'a';
            match mode {
                0 => {
                    add = self.get_random_letter(ConstTypes::VowelsMinusY);
                    if i == length - 1 {
                        add = self.get_random_letter(ConstTypes::Vowels);
                    }
                    mode += 1;
                }
                1 | 2 | 3 => {
                    loop {
                        add = self.get_random_letter(ConstTypes::VowelsMinusY);
                        mode = self.random(4, 7);
                        
                        if {
                            !(add == name.chars().nth(name.len() - 1).unwrap() &&
                            ( add == 'a' || add == 'i' || add == 'u'))
                        } {
                            break;
                        }
                    }
                }
                4 => {
                    add = self.get_random_letter(ConstTypes::Consonants);
                    if i == length - 2 {
                        mode = 0
                    } else {
                        mode += 1;
                    }
                }
                5 | 6 | 7 => {
                    add = self.get_random_letter(ConstTypes::Consonants);
                    mode = self.random(0, 3);
                }
                _ => {
                    add = 'a';
                }
            }
            name.push(add);
        }
        name = capitalize(&name);
        // dbg!(name.clone());
        name.clone()
    }

    pub fn fill_middle(&mut self, start: &str, end: &str) -> String {
        let rand = self.random(0, 5);
        self.fill_middle_with_add(start, end, rand)
    }

    pub fn fill_middle_with_add(&mut self, start: &str, end: &str, add: usize) -> String {
        let mut name = start.to_string();
        for i in 0..add {
            name.push('-');
        }
        self.unscatter(&name)
    }

    fn unscatter(&mut self, name: &str) -> String {
        let mut name = name.to_string().to_lowercase();
        let mut simple_name = self.simplify(&name.clone());
        while !is_readable(&simple_name) {
            simple_name = self.simplify(&name.clone());
        }
        let mut output = String::new();
        for (i, c) in name.chars().enumerate() {
            if c == '-' {
                if simple_name.chars().nth(i).unwrap() == 'v' {
                    if i == name.len() - 1 {
                        output.push(VOWELS.chars().nth(self.random(0, 2)).unwrap());
                    } else {
                        output.push(VOWELS.chars().nth(self.random(0, 2)).unwrap());
                    }
                } else {
                    output.push(CONSONANTS.chars().nth(self.random(0, CONSONANTS.len() - 1)).unwrap());
                }
            } else {
                output.push(c);
            }
        }
        capitalize(&output)
    }

    fn simplify(&mut self, name: &str) -> String {
        let mut output = String::new();
        for (i, c) in name.chars().enumerate() {
            if c == '-' {
                if {
                    let r = self.random(0, 1);
                    r == 0
                } {
                    output.push('v');
                } else {
                    output.push('c');
                }
            } else if is_vowel(c) || i == name.len() - 1 && c == 'y' {
                output.push('v');
            } else {
                output.push('c');
            }
        }
        output
    }

    pub fn seed(&self) -> i64 {
        self.rng.seed()
    }

    fn split(name: &str) -> Vec<&str> {
        name.split_whitespace().collect()
    }

    pub fn random(&mut self, min: usize, max: usize) -> usize {
        let r: usize = (self.rng.next_f64() * (max - min + 1) as f64 + min as f64) as usize;
        r
    }

    fn get_random_letter(&mut self, types: ConstTypes) -> char {
        let distribution: Vec<f64> = match types {
            ConstTypes::Letters => {
                ENGLISH_LETTER_DISTRIBUTION.to_vec()
            }
            ConstTypes::Vowels => {
                ENGLISH_VOWEL_DISTRIBUTION.to_vec()
            }
            ConstTypes::Consonants => {
                ENGLISH_CONSONANT_DISTRIBUTION.to_vec()
            }
            ConstTypes::VowelsMinusY => {
                ENGLISH_VOWEL_DISTRIBUTION_MINUS_Y.to_vec()
            }
        };
        let r = self.rng.next_f64() * array_total(&distribution);
        let mut total = 0.0;
        for (i, val) in distribution.into_iter().enumerate() {
            total = total + val;
            if total >= r {
                match types {
                    ConstTypes::Letters => {
                        return LETTERS.chars().nth(i).unwrap();
                    }
                    ConstTypes::Vowels => {
                        return VOWELS.chars().nth(i).unwrap();
                    }
                    ConstTypes::Consonants => {
                        return CONSONANTS.chars().nth(i).unwrap();
                    }
                    ConstTypes::VowelsMinusY => {
                        return VOWELS_MINUS_Y.chars().nth(i).unwrap();
                    }
                }
            }
        }
        match types {
            ConstTypes::Letters => {
                return LETTERS.chars().nth(0).unwrap();
            }
            ConstTypes::Vowels => {
                return VOWELS.chars().nth(0).unwrap();
            }
            ConstTypes::Consonants => {
                return CONSONANTS.chars().nth(0).unwrap();
            }
            ConstTypes::VowelsMinusY => {
                return VOWELS_MINUS_Y.chars().nth(0).unwrap();
            }
        }
    }

    fn get_mode(&mut self, s: &str, length: usize) -> usize {
        if s.len() == 1 {
            if is_vowel(s.chars().nth(0).unwrap()) {
                return self.random(2, 3);
            } else {
                return self.random(0, 3);
            }
        }
        if is_vowel(s.chars().nth(s.len() - 1).unwrap()) {
            if is_vowel(s.chars().nth(s.len() - 2).unwrap()) {
                return self.random(4, 7);
            } else {
                let r = self.random(0, 3);
                
                if r == 0 {
                    return 1;
                } else {
                    return self.random(4, 7);
                }
            }
        }

        if !is_vowel(s.chars().nth(s.len() - 2).unwrap()) || s.len() + 1 == length {
            return self.random(0, 3);
        }
        let r = self.random(0, 3);
        if r == 0 {
            3
        } else {
            self.random(0, 3)
        }
    }
}


pub fn is_vowel(c: char) -> bool {
    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U' {
        true
    } else {
        false
    }
}

pub fn capitalize(name: &str) -> String {
    let mut name = name.to_string();
    name.chars().next().unwrap().to_uppercase().collect::<String>() + &name[1..]
}

pub fn array_total(arr: &Vec<f64>) -> f64 {
    let mut total = 0.0;
    for i in arr {
        total += *i;
    }
    total
}

pub fn is_readable(name: &str) -> bool {
    let mut v = 0;
    let mut c = 0;
    for i in name.chars() {
        if i == 'v' {
            c = 0;
            v += 1;
        } else {
            v = 0;
            c += 1;
        }
        if c > 2 || v > 2 {
            return false;
        }
    }
    if {
        name.chars().nth(0) == name.chars().nth(name.len() - 1) ||
        name.chars().nth(name.len() - 1) == Some('c') &&
        name.chars().nth(name.len() - 2) == Some('c')
    } {
        return false;
    }
    true
}