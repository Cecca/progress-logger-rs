#[macro_use]
extern crate log;

use std::time::{Duration, Instant};

/// # Examples
///
/// ```
/// use progress_logger::ProgressLogger;
///
/// let mut pl = ProgressLogger::builder().start();
/// let mut cnt = 0;
/// for i in 0..10000 {
///     cnt += 1;
///     pl.up();
/// }
/// pl.stop();
/// ```
pub struct ProgressLogger {
    start: Instant,
    count: u64,
    expected_updates: Option<u64>,
    items: String,
    last_logged: Instant,
    frequency: Duration,
}

impl ProgressLogger {
    pub fn builder() -> ProgressLoggerBuilder {
        ProgressLoggerBuilder {
            expected_updates: None,
            items: None,
            frequency: None,
        }
    }

    fn log(&self) {
        let elapsed = Instant::now() - self.start;
        let throughput = self.count as f64 / elapsed.as_secs_f64();
        if let Some(expected_updates) = self.expected_updates {
            let prediction =
                (expected_updates - self.count) as f64 / throughput;
            info!(
                "{:.2?} {} {}, {:.2} s left ({:.2} {}/s)",
                elapsed, self.count, self.items, prediction, throughput, self.items
            );
        } else {
            info!(
                "{:.2?} {} {} ({:.2} {}/s)",
                elapsed, self.count, self.items, throughput, self.items
            );
        }
    }

    pub fn update<N: Into<u64>>(&mut self, cnt: N) {
        let cnt: u64 = cnt.into();
        self.count += cnt;
        let now = Instant::now();
        if (now - self.last_logged) > self.frequency {
            self.log();
            self.last_logged = now;
        }
    }

    pub fn up(&mut self) {
        self.update(1u64);
    }

    pub fn stop(self) {
        self.log();
    }
}

pub struct ProgressLoggerBuilder {
    expected_updates: Option<u64>,
    items: Option<String>,
    frequency: Option<Duration>,
}

impl ProgressLoggerBuilder {
    pub fn with_expected_updates<N: Into<u64>>(mut self, updates: N) -> Self {
        self.expected_updates = Some(updates.into());
        self
    }
    pub fn with_items_name<S: Into<String>>(mut self, name: S) -> Self {
        self.items = Some(name.into());
        self
    }
    pub fn with_frequency(mut self, freq: Duration) -> Self {
        self.frequency = Some(freq);
        self
    }
    pub fn start(self) -> ProgressLogger {
        let now = Instant::now();
        ProgressLogger {
            start: now,
            count: 0,
            expected_updates: self.expected_updates,
            items: self.items.unwrap_or_else(|| "updates".to_owned()),
            last_logged: now,
            frequency: self.frequency.unwrap_or_else(|| Duration::from_secs(10)),
        }
    }
}

