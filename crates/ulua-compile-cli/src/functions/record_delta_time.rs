extern crate ulua_common;

use ulua_common::functions::get_clock::get_clock;

pub fn record_delta_time(timer: &mut f64) -> f64 {
  let now = get_clock();
  let delta = now - *timer;
  *timer = now;
  delta
}
