use rand::distributions::*;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

#[inline]
pub fn fill_random_buf(len: usize) -> Vec<i64> {
    let mut rng = SmallRng::from_entropy();
    let distr = Uniform::new(3i64, 123_456_789_123);
    let numbers: Vec<i64> = (0..len)
        .map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            distr.sample(&mut rng)
        })
        .collect();
    numbers
}
