//! Source: `Common/include/Luau/TimeTrace.h:148-159` (hand-ported)
//! C++:
//! ```cpp
//! explicit Scope(uint16_t token) : context(getThreadContext())
//! {
//!     if (!FFlag::DebugLuauTimeTracing) return;
//!     context.eventEnter(token);
//! }
//! ```
use crate::{
  FFlag::DebugLuauTimeTracing, functions::get_thread_context::get_thread_context,
  records::scope::Scope,
};

impl Scope {
  pub fn new(token: u16) -> Self {
    let context = get_thread_context();
    let scope = Scope {
      context: context as *mut _,
    };

    if !DebugLuauTimeTracing.get() {
      return scope;
    }

    context.event_enter_u16(token);
    scope
  }
}
