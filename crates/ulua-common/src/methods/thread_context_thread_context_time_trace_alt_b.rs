//! Source: `Common/include/Luau/TimeTrace.h:74-80` (hand-ported)
//! C++ `~ThreadContext() { if (!events.empty()) flushEvents(); releaseThread(*globalContext, this); }`.
use crate::{functions::release_thread::release_thread, records::thread_context::ThreadContext};

impl Drop for ThreadContext {
  fn drop(&mut self) {
    if !self.events.is_empty() {
      self.flush_events();
    }

    // `releaseThread(*globalContext, this)` — 单例上下文自身以 Mutex 保护，
    // 共享 `&` 即可；按 `thread_id` 注销，无需克隆 Arc 或取自身指针。
    release_thread(&self.global_context, self);
  }
}
