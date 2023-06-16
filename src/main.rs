#![feature(asm)]

mod rand;
mod stats;
mod timer;

use std::cmp;
use std::mem::size_of;
use std::time::Duration;
use std::time::Instant;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::rand::fill_random_buf;
use crate::stats::Stats;

// Benchmarking

/// A function that is opaque to the optimizer, to allow benchmarks to
/// pretend to use outputs to assist in avoiding dead-code
/// elimination.
///
/// This function is a no-op, and does not even read from `dummy`.
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
pub fn black_box<T>(dummy: T) -> T {
    // we need to "use" the argument in some way LLVM can't
    // introspect.
    unsafe { asm!("" : : "r"(&dummy)) }
    dummy
}

fn main() {
    let bar = ProgressBar::new(1000);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {bytes}")
            .progress_chars("##-"),
    );
    for _ in 0..1000 {
        bar.inc(1);
        // ...
    }
    bar.finish();

    let sum = timer::bench(&mut || {
        black_box(fill_random_buf(2000000));
    });

    let ns_iter = cmp::max(sum.median as u64, 1);
    let mb_s = size_of::<i64>() * 2000000 * 1000 / ns_iter as usize;
    print!("{} MB/s", mb_s);
}
