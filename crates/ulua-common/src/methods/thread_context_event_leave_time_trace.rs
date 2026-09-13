use crate::{
  functions::get_clock_microseconds::get_clock_microseconds, records::thread_context::ThreadContext,
};

impl ThreadContext {
  pub fn event_leave(&mut self) {
    self.event_leave_u32(get_clock_microseconds());
  }
}
