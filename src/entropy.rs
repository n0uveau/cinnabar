use crate::Frame;

pub trait Extractor {
    fn extract(&self, frames: &[Frame]) -> Vec<u8>;
}

#[derive(Debug, Clone, Default)]
pub struct Diff {
    pub threshold: Option<u64>,
}

impl Diff {
    pub fn with_threshold(threshold: u64) -> Self {
        Self {
            threshold: Some(threshold),
        }
    }

    fn sum(diff: &[u8]) -> u64 {
        diff.iter().map(|&b| b as u64).sum()
    }
}

impl Extractor for Diff {
    fn extract(&self, frames: &[Frame]) -> Vec<u8> {
        frames
            .windows(2)
            .filter_map(|window| {
                let diff = window[0].diff(&window[1]);

                let passes = self.threshold
                    .map_or(true, |t| Self::sum(&diff) >= t);

                passes.then_some(diff)
            })
            .flatten()
            .collect()
    }
}