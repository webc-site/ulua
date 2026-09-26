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

/// token 总量上限（C++ `LUAU_ASSERT(tokens.size() < 64 * 1024)`，保证可装进
/// `u16` 索引）。
const MAX_TOKENS: usize = 64 * 1024;

/// cpp `TimeTrace` 镜像工件（`Common/src/TimeTrace.cpp:115-123`），sync-cpp 维护；
/// 仅本 crate 打点机制消费（`create_scope_data` 与 flush token），降 `pub(crate)`。
pub(crate) fn create_token(
  context: &GlobalContext,
  name: &'static str,
  category: &'static str,
) -> u16 {
  // `scoped_lock lock(context.mutex)` — the lock guards the mutable state.
  let mut state = context.lock_state();

  LUAU_ASSERT!(state.tokens.len() < MAX_TOKENS);

  state.tokens.push(Token { name, category });
  (state.tokens.len() - 1) as u16
}
