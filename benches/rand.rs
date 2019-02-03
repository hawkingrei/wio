#![feature(test)]
extern crate test;

const RAND_BENCH_N: u64 = 1000;

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
        let mut accum = Duration::new(0, 0);
        for _ in 0..RAND_BENCH_N {
            let _x: i64 = distr.sample(&mut rng);
        }
        accum
    });
    b.bytes = size_of::<i64>() as u64 * RAND_BENCH_N;
}
