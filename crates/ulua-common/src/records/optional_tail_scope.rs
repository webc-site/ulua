use crate::{
  FFlag::DebugLuauTimeTracing, functions::get_clock_microseconds::get_clock_microseconds,
  records::thread_context::ThreadContext,
};

#[derive(Debug)]
pub struct OptionalTailScope {
  pub(crate) context: *mut ThreadContext,
  pub(crate) token: u16,
  pub(crate) threshold: u32,
  pub(crate) microsec: u32,
  pub(crate) pos: u32,
}

impl Drop for OptionalTailScope {
  fn drop(&mut self) {
    if DebugLuauTimeTracing.get() && !self.context.is_null() {
      unsafe {
        let ctx = &mut *self.context;
        if self.pos == ctx.events.len() as u32 {
          let curr = get_clock_microseconds();
          if curr.wrapping_sub(self.microsec) > self.threshold {
            ctx.event_enter_u16_u32(self.token, self.microsec);
            ctx.event_leave_u32(curr);
          }
        }
      }
    }
  }
}
