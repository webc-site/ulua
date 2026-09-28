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
//! 与 `create_thread` 对应：注销的是 `threadId`（cpp 比对的是上下文指针，两者
//! 在"唯一标识一个活跃线程上下文"上等价）。
use crate::records::global_context::GlobalContext;

/// 从全局登记表移除 `thread_id`，等价 cpp `releaseThread`。
///
/// cpp `TimeTrace` 镜像工件（`Common/src/TimeTrace.cpp:134-140`），sync-cpp 维护；
/// 仅本 crate 打点机制（`ThreadContext::drop`）消费，降 `pub(crate)`。
pub(crate) fn release_thread(context: &GlobalContext, thread_id: u32) {
  // `scoped_lock lock(context.mutex)`
  let mut state = context.lock_state();

  if let Some(pos) = state
    .threads
    .iter()
    .position(|registered| *registered == thread_id)
  {
    state.threads.remove(pos);
  }
}
