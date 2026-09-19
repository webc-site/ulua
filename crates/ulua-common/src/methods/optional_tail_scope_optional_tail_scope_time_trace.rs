use crate::{
  fflag::DebugLuauTimeTracing,
  functions::{
    get_clock_microseconds::get_clock_microseconds, get_thread_context::get_thread_context,
  },
  records::optional_tail_scope::OptionalTailScope,
};

impl OptionalTailScope {
  pub fn new(token: u16, threshold: u32) -> Self {
    let context = get_thread_context();
    let mut scope = Self {
      context,
      token,
      threshold,
      microsec: 0,
      pos: 0,
    };

    if DebugLuauTimeTracing.get() {
      // SAFETY: `context` 是本线程的 TLS/provider 实例，线程存活期内有效。
      scope.pos = unsafe { (*context).events.len() } as u32;
      scope.microsec = get_clock_microseconds();
    }

    scope
  }
}
