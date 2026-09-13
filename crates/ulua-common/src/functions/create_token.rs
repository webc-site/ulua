//! Source: `Common/src/TimeTrace.cpp:115-123` (hand-ported)
//! C++:
//! ```cpp
//! uint16_t createToken(GlobalContext& context, const char* name, const char* category)
//! {
//!     std::scoped_lock lock(context.mutex);
//!     LUAU_ASSERT(context.tokens.size() < 64 * 1024);
//!     context.tokens.push_back({name, category});
//!     return uint16_t(context.tokens.size() - 1);
//! }
//! ```
use crate::{
  macros::luau_assert::LUAU_ASSERT,
  records::{global_context::GlobalContext, token::Token},
};

pub fn create_token(context: &GlobalContext, name: &'static str, category: &'static str) -> u16 {
  // `scoped_lock lock(context.mutex)` — the lock guards the mutable state.
  let mut state = context
    .state
    .lock()
    .expect("TimeTrace GlobalContext mutex poisoned");

  LUAU_ASSERT!(state.tokens.len() < 64 * 1024);

  state.tokens.push(Token { name, category });
  (state.tokens.len() - 1) as u16
}
