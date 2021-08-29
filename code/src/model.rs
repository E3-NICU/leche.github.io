use std::ops::Range;

// These parameters represent the linear fit of the data calculated in R.
// The first is the model for 360 watt and the second the one for 90 watt.
// These can be freely changed upon a new model
const SECONDS: [f64; 2] = [2.1892, 1.8395];
const VOLUME: [f64; 2] = [-0.4125, -1.2162];
const INTERCEPT: [f64; 2] = [12.2044, 11.5714];
const WATT: [u64; 2] = [360, 90];
const ERROR: [f64; 2] = [2.0, 5.0];

// The temperature decline for every minute
const DECAY: f64 = 0.2;

// The target temperature
const TARGET: f64 = 34.0;

// A struct for the return values of the model
#[derive(Clone, Debug, PartialEq)]
pub struct ModelResult {
    pub seconds: u64,
    pub estimate: Range<f64>,
    pub watt: u64,
}

pub fn exec_model(volume: u64, start: u64, time: u64) -> ModelResult {
    let index = if volume < 20 { 1usize } else { 0 };

    // Calculate the difference accounting for temperature decline over time
    let difference = TARGET - (start as f64) + (time as f64) * DECAY;

    // The exact calculation of seconds
    let seconds = (difference - INTERCEPT[index] - VOLUME[index] * (volume as f64)) / SECONDS[index];

    // Round the seconds to show them on the UI
    let rounded = seconds.round();

    // Calculate the expected value based on the rounded seconds
    let expected = INTERCEPT[index] + SECONDS[index] * rounded + VOLUME[index] * (volume as f64) + (start as f64);

    // For now the error is just a flat range depending on the model
    let estimate = (expected - ERROR[index])..(expected + ERROR[index]);

    ModelResult { seconds: rounded as u64, estimate, watt: WATT[index] }
}