//! Source: `Common/src/TimeTrace.cpp:134-140` (hand-ported)
//! C++:
//! ```cpp
//! void releaseThread(GlobalContext& context, ThreadContext* threadContext)
//! {
//!     std::scoped_lock lock(context.mutex);
//!     if (auto it = std::find(context.threads.begin(), context.threads.end(), threadContext); it != context.threads.end())
//!         context.threads.erase(it);
//! }
//! ```
//!
//! 偏差（行为等价）：注册身份是 `thread_id`（见
//! [`crate::functions::create_thread`]），故按 `thread_context.thread_id`
//! 查找；未注册（feature 关闭时 `thread_id == 0`）则找不到，静默返回，
//! 与 C++ `find` 未命中路径一致。
use crate::records::{global_context::GlobalContext, thread_context::ThreadContext};

pub fn release_thread(context: &GlobalContext, thread_context: &ThreadContext) {
  let mut state = context.lock_state();

  let thread_id = thread_context.thread_id;
  if let Some(pos) = state.threads.iter().position(|&id| id == thread_id) {
    state.threads.remove(pos);
  }
}
