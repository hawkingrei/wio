mod rand;
mod timer;

use std::mem::size_of;
use std::time::Duration;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::rand::fill_random_buf;

fn throughput(dur: Duration, bytes: usize) -> u64 {
    let ns_iter = dur.as_secs() * 1_000_000_000 + (dur.subsec_nanos() as u64);
    bytes as u64 * 1000 / ns_iter
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
    let dur = timer::bench(10, || {
        fill_random_buf(2000000);
        1000
    });
    print!("{} MB/s", throughput(dur, size_of::<i64>() * 2000000));
}
