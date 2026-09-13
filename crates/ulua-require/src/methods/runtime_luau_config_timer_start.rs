use std::time::{Duration, Instant};

use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;

impl RuntimeLuauConfigTimer {
  pub fn start(&mut self, timeout_ms: i32) {
    self.start_time = Instant::now();
    self.timeout_duration = (timeout_ms >= 0).then(|| Duration::from_millis(timeout_ms as u64));
  }
}
