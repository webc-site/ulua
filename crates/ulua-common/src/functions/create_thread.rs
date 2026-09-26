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
//! cpp 把 `ThreadContext*` 记进全局表：C++ 里上下文生命周期由其宿主线程的
//! `thread_local` 保管，且 `~ThreadContext` 必然 `releaseThread` 配对。Rust 的
//! move 语义下"注册自身地址"会在构造返回时当场悬垂，故全局表只登记 `threadId`
//! （见 `records::global_context`），本函数不再接受上下文指针。
use crate::records::global_context::GlobalContext;

/// 分配一个新的线程 id 并登记为活跃上下文，等价 cpp `createThread`。
///
/// cpp `TimeTrace` 镜像工件（`Common/src/TimeTrace.cpp:125-132`），sync-cpp 维护；
/// 仅本 crate 打点机制（`ThreadContext` 构造）消费，降 `pub(crate)`。
pub(crate) fn create_thread(context: &GlobalContext) -> u32 {
  // `scoped_lock lock(context.mutex)`
  let mut state = context.lock_state();

  state.next_thread_id += 1;
  let thread_id = state.next_thread_id;
  state.threads.push(thread_id);

  thread_id
}
