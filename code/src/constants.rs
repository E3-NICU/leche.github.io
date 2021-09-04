use std::ops::Range;

// Change these values to have different min and max for the sliders
pub const TEMP_RANGE: Range<u64> = 0..25;
pub const VOLUME_RANGE: Range<u64> = 5..70;
pub const WAIT_RANGE: Range<u64> = 0..10;

// Do not change these, these indicate the steps on their respective slider
pub const TEMP_STEPS: u64 = TEMP_RANGE.end - TEMP_RANGE.start;
pub const VOLUME_STEPS: u64 = VOLUME_RANGE.end - VOLUME_RANGE.start;
pub const WAIT_STEPS: u64 = WAIT_RANGE.end - WAIT_RANGE.start;

// These are the values that represent the choice
pub const FRIDGE_DEFAULT: u64 = 9;
pub const SYNTHETIC_DEFAULT: u64 = 7;
pub const ROOM_DEFAULT: u64 = 23;
pub const IMMEDIATELY_DEFAULT: u64 = 3;

// These are the values that are default for the sliders
pub const VOLUME_INITIAL: u64 = 40;
pub const MEASURED_INITIAL: u64 = 9;
pub const LATER_INITIAL: u64 = 3;

// A struct for the parameters of the linear model
pub struct LinearParams {
    pub seconds_est: f64,
    pub volume_est: f64,
    pub intercept: f64,
    pub watt: u64,
    pub error: f64,
}

// These parameters represent the linear fit of the data calculated in R.
pub const PARAMS90: LinearParams = LinearParams {
    seconds_est: 1.8395,
    volume_est: -1.2162,
    intercept: 11.5714,
    watt: 90,
    error: 5.0,
};

// These parameters represent the linear fit of the data calculated in R.
pub const PARAMS360: LinearParams = LinearParams {
    seconds_est: 2.1892,
    volume_est: -0.4125,
    intercept: 12.2044,
    watt: 360,
    error: 2.0,
};

// The temperature decline for every minute
pub const DECAY: f64 = 0.2;

// The target temperature
pub const TARGET: f64 = 34.0;

// The parameters are based on experiments with water so we use a small correction term.
// https://www.engineeringtoolbox.com/specific-heat-fluids-d_151.html
pub const CORRECTION: f64 = 0.94;

// The break point for switching the models
pub const BREAK: u64 = 20;