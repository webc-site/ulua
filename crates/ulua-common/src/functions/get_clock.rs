use std::sync::OnceLock;

use crate::functions::{
  get_clock_period::get_clock_period, get_clock_timestamp::get_clock_timestamp,
};

pub fn get_clock() -> f64 {
  static PERIOD: OnceLock<f64> = OnceLock::new();
  static START: OnceLock<f64> = OnceLock::new();

  let period = *PERIOD.get_or_init(get_clock_period);
  let start = *START.get_or_init(get_clock_timestamp);

  (get_clock_timestamp() - start) * period
}
