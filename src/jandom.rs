// This is a simplified version of the Jandom library.
// It's the most accurate version to the original source code that I could find.
// But it doesn't compile.
// So I'm just going to use what I need.

use std::sync::atomic::{AtomicI64, Ordering};

const MULTIPLIER: i64 = 0x5DEECE66D;
const INCREMENT: i64 = 0xB;
const MASK: i64 = (1 << 48) - 1;

pub struct Random {
    seed: AtomicI64,
}
impl Random {
    pub fn new(seed: i64) -> Random {
        Random {
            seed: AtomicI64::new(Self::initialize_state(seed))
        }
    }

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
        }
    }

    fn initialize_state(seed: i64) -> i64 {
        seed ^ MULTIPLIER & MASK
    }

    pub fn next_f64(&mut self, bytes: &mut [i8]) {
        let max = bytes.len() & !0x3;

        for i in (0..max).step_by(4) {
            let random = self.next(32);

        }
    }
}