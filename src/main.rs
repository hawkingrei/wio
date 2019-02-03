mod rand;
mod timer;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

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
}
