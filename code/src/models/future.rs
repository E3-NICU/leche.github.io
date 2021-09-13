#![allow(unused, unreachable_code, unused_variables)]
// When implementing this remove the #[allow(unused, unreachable_code, unused_variables)] line!

use std::ops::Range;

use crate::models::base::Model;
use crate::constants::{BREAK, PARAMS90, PARAMS360, CORRECTION};


#[derive(Default)]
pub struct FutureModel;

impl Model for FutureModel {
    fn calc_seconds_and_watt(&self, volume: u64, start: u64, target: f64) -> (f64, u64) {
        let seconds = todo!(); // e.g. (volume * time + start) * 0.1
        let watt = todo!(); // e.g. 360

        (seconds, watt)
    }

    fn calc_expected(&self, volume: u64, start: u64, seconds: u64) -> Range<f64> {
        let expected = todo!(); // e.g. 32.0 .. 37.0

        expected
    }
}
