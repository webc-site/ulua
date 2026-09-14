use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;

impl RuntimeLuauConfigTimer {
  pub fn is_finished(&self) -> bool {
    self
      .timeout_duration
      .is_some_and(|timeout| self.start_time.elapsed() >= timeout)
  }
}
