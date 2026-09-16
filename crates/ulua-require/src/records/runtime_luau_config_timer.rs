use std::time::{Duration, Instant};
#[derive(Debug, Clone, Copy)]
pub struct RuntimeLuauConfigTimer {
  pub(crate) start_time: Instant,
  pub(crate) timeout_duration: Option<Duration>,
}
