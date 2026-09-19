//! Source: `Common/src/TimeTrace.cpp:125-132` (hand-ported)
//! C++:
//! ```cpp
//! uint32_t createThread(GlobalContext& context, ThreadContext* threadContext)
//! {
//!     std::scoped_lock lock(context.mutex);
//!     context.threads.push_back(threadContext);
//!     return ++context.nextThreadId;
//! }
//! ```
//!
//! 偏差（行为等价）：C++ 以 `ThreadContext*` 作注册身份；Rust 的
//! `ThreadContext` 按值移动，`this` 指针在构造与析构之间不稳定，故
//! 以唯一的 `thread_id` 注册（见 [`crate::records::global_context`]），
//! 注销时按 `thread_id` 配对。
use crate::records::global_context::GlobalContext;

pub fn create_thread(context: &GlobalContext) -> u32 {
  let mut state = context.lock_state();

  state.next_thread_id += 1;
  let thread_id = state.next_thread_id;
  state.threads.push(thread_id);
  thread_id
}
