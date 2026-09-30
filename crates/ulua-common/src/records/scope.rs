//! cpp `struct Scope`（`Common/include/Luau/TimeTrace.h:148-168`）：RAII 打点区间。
//!
//! 上游成员 `ThreadContext& context` 在 Rust 侧不必保存：构造与析构各经
//! `with_thread_context` 在借用期内取用同一份线程局部上下文，因此没有长期持有的
//! 引用/裸指针，也没有 null 分支。
use crate::{
  fflag::DebugLuauTimeTracing, functions::with_thread_context::with_thread_context,
  records::thread_context::ENABLED,
};

/// `LUAU_TIMETRACE_SCOPE` 的 RAII 句柄。
///
/// cpp 镜像工件（`TimeTrace.h` 的 `struct Scope`），sync-cpp 维护；保留 `pub`：
/// 宏启用形态在下游 crate 展开时以 `let _scope = Scope::new(..)` 形态构造（宏 ABI 面）。
#[derive(Debug)]
pub struct Scope;

impl Scope {
  /// cpp `Scope(uint16_t token)`（`TimeTrace.h:148-159`）：成员初始化先取线程上下文
  /// （可能触发其惰性构造与登记），随后才按 `DebugLuauTimeTracing` 决定是否记 Enter
  /// 事件。cpp 镜像工件，sync-cpp 维护；`pub` = 宏 ABI 面（下游展开直接调用）。
  pub fn new(token: u16) -> Self {
    if ENABLED {
      with_thread_context(|context| {
        if DebugLuauTimeTracing.get() {
          context.event_enter(token);
        }
      });
    }
    Scope
  }
}

impl Drop for Scope {
  fn drop(&mut self) {
    if ENABLED && DebugLuauTimeTracing.get() {
      with_thread_context(|context| context.event_leave());
    }
  }
}
