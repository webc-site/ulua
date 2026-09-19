use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;

impl RuntimeLuauConfigTimer {
  pub(crate) fn is_finished(&self) -> bool {
    self
      .timeout_duration
      .get()
      .is_some_and(|timeout| self.start_time.get().elapsed() >= timeout)
  }
}
