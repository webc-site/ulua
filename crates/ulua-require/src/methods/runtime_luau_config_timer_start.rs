use std::time::{Duration, Instant};

use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;

impl RuntimeLuauConfigTimer {
  pub(crate) fn start(&self, timeout_ms: i32) {
    self.start_time.set(Instant::now());
    self
      .timeout_duration
      .set((timeout_ms >= 0).then(|| Duration::from_millis(timeout_ms as u64)));
  }
}
