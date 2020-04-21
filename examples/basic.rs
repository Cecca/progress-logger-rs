#[macro_use] extern crate log;

use progress_logger::ProgressLogger;
use std::time::Duration;

fn main() {
    env_logger::init();

    let n = std::env::args()
        .nth(1)
        .expect("pass n on the command line")
        .parse::<u64>()
        .expect("n should be an integer");

    info!("Light updates");
    let mut pl = ProgressLogger::builder()
        .with_expected_updates(n)
        .with_frequency(Duration::from_secs(1))
        .with_items_name("nodes")
        .start();

    for _ in 0..n {
        pl.update_light(1u32);
    }
    pl.stop();

    info!("Full updates");
    let mut pl = ProgressLogger::builder()
        .with_expected_updates(n)
        .with_frequency(Duration::from_secs(1))
        .with_items_name("nodes")
        .start();
    for _ in 0..n {
        pl.update(1u32);
    }
    pl.stop();

    info!("Up shorthand");
    let mut pl = ProgressLogger::builder()
        .with_expected_updates(n)
        .with_frequency(Duration::from_secs(1))
        .with_items_name("nodes")
        .start();
    for _ in 0..n {
        pl.up();
    }
    pl.stop();
}
