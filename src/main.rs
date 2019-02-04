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
        fill_random_buf(2000000);
        0
    });

    let ns_iter = cmp::max(sum.median as u64, 1);
    let mb_s = size_of::<i64>() * 2000000 * 1000 / ns_iter as usize;
    print!("{} MB/s", mb_s);
}
