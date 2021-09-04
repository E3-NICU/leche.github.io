use std::ops::Range;

use crate::models::base::Model;
use crate::constants::{BREAK, PARAMS90, PARAMS360, CORRECTION};


#[derive(Default)]
pub struct LinearModel;

impl Model for LinearModel {
    fn calc_seconds_and_watt(&self, volume: u64, start: u64, target: f64) -> (f64, u64) {
        let params = if volume < BREAK { PARAMS90 } else { PARAMS360 };

        let difference = target - (start as f64);

        // The exact calculation of seconds
        let seconds = CORRECTION
            * (difference - params.intercept - params.volume_est * (volume as f64))
            / params.seconds_est;

        (seconds, params.watt)
    }

    fn calc_expected(&self, volume: u64, start: u64, seconds: u64) -> Range<f64> {
        let params = if volume < BREAK { PARAMS90 } else { PARAMS360 };

        // Calculate the expected value based on the rounded seconds
        let expected = params.intercept
            + params.seconds_est / CORRECTION * seconds as f64
            + params.volume_est * (volume as f64)
            + (start as f64);

        // For now the error is just a flat range depending on the model
        (expected - params.error)..(expected + params.error)
    }
}
