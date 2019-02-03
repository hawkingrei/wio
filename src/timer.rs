use std::cmp;

use std::time::Duration;
use std::time::Instant;

pub fn bench<T, F>(trials: usize, f: F) -> Duration
where
    F: Fn() -> T,
{
    let start = Instant::now();
    for _ in 0..trials {
        let _keep = f();
    }
    start.elapsed()
}
