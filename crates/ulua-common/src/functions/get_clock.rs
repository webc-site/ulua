use std::sync::OnceLock;

use crate::functions::{
  get_clock_period::get_clock_period, get_clock_timestamp::get_clock_timestamp,
};

pub fn get_clock() -> f64 {
  // 周期与起始计数同源同刻初始化，收进单个 OnceLock。
  static STATE: OnceLock<(f64, f64)> = OnceLock::new();

  let (period, start) = *STATE.get_or_init(|| (get_clock_period(), get_clock_timestamp()));

  (get_clock_timestamp() - start) * period
}
