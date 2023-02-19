use std::time::{Duration, Instant};

use crate::error::{MyError, MyResult};

pub struct Report {
    origin: Vec<u8>,
    compressed: Vec<u8>,
    time_elapsed: Duration,
}

impl Report {
    const REPORT_FILE_BYTES: usize = 512;
    pub fn finalize(&self) -> String {
        format!(
            "Origin: {:?}\nCompressed: {:?}\nCompressed/Origin = {} / {}\nCompression rate: {:.3}\nTime to work: {:?}",
            self.origin
                .iter()
                .take(Self::REPORT_FILE_BYTES)
                .collect::<Vec<_>>(),
            self.compressed
                .iter()
                .take(Self::REPORT_FILE_BYTES)
                .collect::<Vec<_>>(),
            self.compressed.len(), self.origin.len(),
            self.compressed.len() as f64 / self.origin.len() as f64,
            self.time_elapsed,
        )
    }
}

pub struct ReportBuilder {
    origin: Option<Vec<u8>>,
    compressed: Option<Vec<u8>>,
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

    pub fn set_origin(&mut self, origin: Vec<u8>) {
        self.origin = Some(origin);
    }

    pub fn set_compressed(&mut self, compressed: Vec<u8>) {
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
