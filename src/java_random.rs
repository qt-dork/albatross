// This is a simplified version of the Jandom library,
// located at: https://github.com/kallekankaanpaa/jandom
// It's the most accurate version to the original java
// source code that I could find, But it doesn't compile.
// So I'm just going to use what I need.

use std::num::Wrapping;

pub const MULTIPLIER: Wrapping<i64> = Wrapping(0x5DEECE66D);
pub const ADDEND: Wrapping<i64> = Wrapping(0xB);
pub const MASK: Wrapping<i64> = Wrapping((1 << 48) - 1);

const F32_DIV: f32 = (1u32 << 24) as f32;
const F64_DIV: f64 = (1u64 << 53) as f64;

#[derive(Debug, Copy, Clone)]
pub struct Random {
    state: Wrapping<i64>,
}

impl Random {
    /// Creates a new random number generator using a single [u64] seed.
    /// 
    /// This has the same effect as calling the constructor with seed param in Java.
    pub fn new(seed: i64) -> Self {
        Random {
            state: Wrapping((seed) ^ MULTIPLIER.0) & MASK,
        }
    }

    /// Sets the seed to `seed`. This is equivalent to `Random::new`
    pub fn set_seed(&mut self, seed: i64) {
        *self = Random::new(seed);
    }

    /// Steps the RNG, returning up to 48 bits.
    /// 
    /// # Panics
    /// If the amount of requested bits is over 48, this function panics. Use next_i64/next_u64 instead, or multiple calls.
    pub fn next(&mut self, bits: u8) -> i32 {
        if bits > 48 {
            panic!("Too many bits!")
        }

        self.state = (self.state * MULTIPLIER + ADDEND) & MASK;

        (self.state.0 as u64 >> (48 - bits)) as i32
    }

    /// Fills the byte array with random bytes.
    pub fn next_bytes(&mut self, bytes: &mut [u8]) {
        for chunk in bytes.chunks_mut(4) {
            let mut block = self.next_u32();

            for item in chunk {
                *item = (block & 0xFF) as u8;
                block >>= 8;
            }
        }
    }

    /// Returns a uniformly distributed signed 32-bit integer.
    pub fn next_i32(&mut self) -> i32 {
        self.next(32) as i32
    }

    /// Returns a uniformly distributed unsigned 32-bit integer.
    pub fn next_u32(&mut self) -> u32 {
        self.next(32) as u32
    }

    /// Returns a positive random number in the range [0, max), up to 2^31.
    /// The range of the return value is represented by the value `0 <= value < max`.
    /// A maximum of less than 1 is invalid because then no value would satisfy the range.
    /// 
    /// # Panics
    /// If `max` is less than 1, this function panics.
    pub fn next_i32_bound(&mut self, max: i32) -> i32 {
        if max <= 0 {
            panic!("Maximum must > 0")
        }

        if (max as u32).is_power_of_two() {
            let max = max as i64;
            return ((max.wrapping_mul(self.next(31) as i64)) >> 31) as i32;
        }

        let mut bits = self.next(31);
        let mut val = bits % max;

        while bits.wrapping_sub(val).wrapping_add(max - 1) < 0 {
            bits = self.next(31);
            val = bits % max;
        }

        val
    }

    /// Returns a positive random number in the range [0, max), up to 2^31.
    /// The range of the return value is represented by the value `0 <= value < max`.
    /// A maximum of 0 is invalid because then no value would satisfy the range.
    /// Maximums of 2^31 or greater are not supported in Java.
    /// 
    /// # Panics
    /// If `max` reinterpreted as a signed 32-bit integer is less than 1, this function panics.
    pub fn next_u32_bound(&mut self, max: u32) -> u32 {
        self.next_i32_bound(max as i32) as u32
    }

    /// Returns a uniformly distributed signed 64-bit integer.
    pub fn next_i64(&mut self) -> i64 {
        ((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
    }

    /// Returns a uniformly distributed unsigned 64-bit integer.
    pub fn next_u64(&mut self) -> u64 {
        self.next_i64() as u64
    }

    /// Returns a boolean value that has an equal chance of being true or false.
    pub fn next_bool(&mut self) -> bool {
        self.next(1) == 1
    }

    /// Returns a f32 uniformly distributed between 0.0 and 1.0.
    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / F32_DIV
    }

    /// Returns a f64 uniformly distributed between 0.0 and 1.0.
    pub fn next_f64(&mut self) -> f64 {
        let high = (self.next(26) as i64) << 27;
        let low = self.next(27) as i64;

        (high.wrapping_add(low) as f64) / F64_DIV
    }


}