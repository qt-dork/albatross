// This is a simplified version of the Jandom library,
// located at: https://github.com/kallekankaanpaa/jandom
// It's the most accurate version to the original java
// source code that I could find, But it doesn't compile.
// So I'm just going to use what I need.

use std::fmt;
//use std::sync::atomic::{AtomicI64, Ordering};

const MULTIPLIER: i64 = 0x5DEECE66D;
const INCREMENT: i64 = 0xB;
const MASK: i64 = (1 << 48) - 1;

#[derive(Copy, Clone)]
pub struct Random {
    seed: i64,
}

impl Random {
    /// Creates a new random number generator using a single [i64] seed.
    /// 
    /// This has the same effect as calling the constructor with seed param in Java.
    pub fn new(seed: i64) -> Random {
        Random {
            seed: i64::new(Self::initialize_state(seed))
        }
    }

    /// Advances the RNG by one and returns random bits.
    fn next(&mut self, bits: u8) -> i32 {
        if bits > 32 {
            panic!("Cannot generate more than 32 bits");
        }

        let mut previous_state = self.seed.load(Ordering::Acquire);
        loop {
            let new_state = previous_state
                .wrapping_mul(MULTIPLIER)
                .wrapping_add(INCREMENT)
                & MASK;
            // Using is weak since it allows optimizations on certain (e.g. ARM) architectures
            match self.seed.compare_exchange_weak(
                previous_state,
                new_state,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return (new_state >> (48 - bits)) as i32,
                Err(current_state) => previous_state = current_state,
            }
        }
    }

    /// Calculates the initial state from a seed.
    fn initialize_state(seed: i64) -> i64 {
        seed ^ MULTIPLIER & MASK
    }

    /// Returns the next pseudorandom, uniformly distributed 
    /// [f64] value between 0.0 and 1.0 from this random
    /// number generator's sequence.
    /// 
    /// The general contract of `next_f64` is that one [f64]
    /// value, chosen (approximately) uniformly from the
    /// range 0.0 (inclusive) to 1.0 (exclusive), is
    /// pseudorandomly generated and returned.
    pub fn next_f64(&mut self) -> f64 {
        ((((self.next(26) as i64) << 27) + (self.next(27) as i64)) as f64) / ((1_i64 << 53) as f64)
    }
}

/// Implemented custom Debug in order to prevent users from leaking the internal state.
impl fmt::Debug for Random {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Random number generator implemented with the same algorithm as java.util.Random")
    }
}

// static SEED_UNIQUFIER: SyncLazy<AtomicI64> = SyncLazy::new(|| AtomicI64::new(8682522807148012));

// /// The default implementation represents the Java Random constructor with no params.
// impl Default for Random {
//     fn default() -> Self {
//         use std::time::{SystemTime, UNIX_EPOCH};
//         const MULTIPLIER: i64 = 1181783497276652981;

//         let mut current = SEED_UNIQUFIER.load(Ordering::Acquire);
//         loop {
//             let new = current.wrapping_mul(MULTIPLIER);
//             match SEED_UNIQUFIER.compare_exchange_weak(
//                 current,
//                 new,
//                 Ordering::AcqRel,
//                 Ordering::Relaxed,
//             ) {
//                 Ok(_) => {
//                     let elapsed = SystemTime::now()
//                         .duration_since(UNIX_EPOCH)
//                         .expect("SystemTime returned value earlier than UNIX_EPOCH");
//                     return Self::new(new ^ (elapsed.as_nanos() as i64));
//                 },
//                 Err(uniquifier) => current = uniquifier,
//     }
// }
