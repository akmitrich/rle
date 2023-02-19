use std::time::{Duration, Instant};

use crate::error::{MyError, MyResult};

pub struct Report {
    origin: usize,
    compressed: usize,
    time_elapsed: Duration,
}

impl Report {
    pub fn finalize(&self) -> String {
        format!(
            "Compressed/Origin = {} / {}\nCompression rate: {:.3}\nTime to work: {:?}",
            self.compressed,
            self.origin,
            self.compressed as f64 / self.origin as f64,
            self.time_elapsed,
        )
    }
}

pub struct ReportBuilder {
    origin: Option<usize>,
    compressed: Option<usize>,
    start: Instant,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self {
            origin: None,
            compressed: None,
            start: Instant::now(),
        }
    }

    pub fn set_origin(&mut self, origin: usize) {
        self.origin = Some(origin);
    }

    pub fn set_compressed(&mut self, compressed: usize) {
        self.compressed = Some(compressed);
    }

    pub fn finalize(self) -> MyResult<Report> {
        let origin = self
            .origin
            .ok_or_else(|| Box::new(MyError::InvalidReport))?;
        let compressed = self
            .compressed
            .ok_or_else(|| Box::new(MyError::InvalidReport))?;
        Ok(Report {
            origin,
            compressed,
            time_elapsed: Instant::now().duration_since(self.start),
        })
    }
}
