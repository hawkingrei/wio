#![feature(test)]
extern crate test;

const RAND_BENCH_N: u64 = 2000000;

use std::mem::size_of;
use std::time::Duration;
use test::test::Bencher;

use rand::distributions::*;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

#[bench]
fn distr_uniform_i64(b: &mut Bencher) {
    let mut rng = SmallRng::from_entropy();
    let distr = Uniform::new(3i64, 123_456_789_123);
    b.iter(|| {
        let mut rng = SmallRng::from_entropy();
        let distr = Uniform::new(3i64, 123_456_789_123);
        let numbers: Vec<i64> = (0..RAND_BENCH_N)
        .map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            distr.sample(&mut rng)
        })
        .collect();        
    });
    b.bytes = size_of::<i64>() as u64 * RAND_BENCH_N;
}
