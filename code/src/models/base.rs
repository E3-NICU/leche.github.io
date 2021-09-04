use std::ops::Range;
use crate::constants::{TARGET, DECAY};

// A struct for the return values of the model
#[derive(Clone, Debug, PartialEq)]
pub struct ModelResult {
    pub seconds: u64,
    pub expected: Range<f64>,
    pub watt: u64,
}

pub trait Model {
    fn calc_seconds_and_watt(&self, volume: u64, start: u64, target: f64) -> (f64, u64);
    fn calc_expected(&self, volume: u64, start: u64, seconds: u64) -> Range<f64>;

    fn calc(&self, volume: u64, start: u64, wait: u64) -> ModelResult {
        // Calculate the target accounting for temperature decline over time
        let target = TARGET + wait as f64 * DECAY;

        // The exact calculation of seconds and the amount of watts
        let (seconds, watt) = self.calc_seconds_and_watt(volume, start, target);

        // Round the seconds to show them on the UI
        let rounded = seconds.round() as u64;

        // Calculate the expected value based on the rounded seconds
        let expected = self.calc_expected(volume, start, rounded);

        ModelResult {
            seconds: rounded,
            expected,
            watt,
        }
    }
}