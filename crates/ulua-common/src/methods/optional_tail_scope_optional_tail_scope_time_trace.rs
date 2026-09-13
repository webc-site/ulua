use crate::{
  FFlag::DebugLuauTimeTracing,
  functions::{
    get_clock_microseconds::get_clock_microseconds, get_thread_context::get_thread_context,
  },
  records::optional_tail_scope::OptionalTailScope,
};

impl OptionalTailScope {
  pub fn new(token: u16, threshold: u32) -> Self {
    let context = get_thread_context();
    let mut scope = Self {
      context: context as *mut _,
      token,
      threshold,
      microsec: 0,
      pos: 0,
    };

    if DebugLuauTimeTracing.get() {
      scope.pos = context.events.len() as u32;
      scope.microsec = get_clock_microseconds();
    }

    scope
  }
}
