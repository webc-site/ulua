//! Source: `Common/src/TimeTrace.cpp:261-269` (hand-ported)
//! C++:
//! ```cpp
//! ThreadContext& getThreadContext()
//! {
//!     // Check custom provider that which might implement a custom TLS
//!     if (auto provider = threadContextProvider())
//!         return provider();
//!     thread_local ThreadContext context;
//!     return context;
//! }
//! ```
//!
//! cpp 交出 `ThreadContext&` 让调用方（`Scope`）长期持有；
//! Rust 里同一线程上并存多个 such 引用就等于并存别名（要么 `&mut` 冲突，要么靠
//! 裸指针绕过借用检查）。这里改为"在闭包内使用上下文"：借用期即闭包作用域，
//! 唯一可变别名由 `RefCell` 在运行期校验、由编译器在类型上约束，不再有
//! `*mut`/null 检查。
//!
//! 未移植项：`threadContextProvider()` 自定义 TLS 钩子。它是 `extern "C"` 的
//! `ThreadContext* (*)()` 函数指针槽，唯一用途是让非 Rust 的 host（Roblox 端）
//! 把自己的线程局部存储接进来；本 workspace 全为 Rust、无任何设置点，保留只会
//! 重新引入上述裸指针别名问题。若将来需要 host 注入上下文，应按 Rust 习惯设计
//! 成 trait 对象 + 生命周期约束，而不是裸指针槽。
use std::cell::RefCell;

use crate::records::thread_context::ThreadContext;

thread_local! {
  /// 线程局部上下文，等价 cpp `thread_local ThreadContext context`。
  /// 首次访问时惰性构造（向 `GlobalContext` 登记线程 id），线程退出时随 TLS
  /// 析构：`ThreadContext::drop` 落盘残余事件并注销登记。
  static CONTEXT: RefCell<ThreadContext> = RefCell::new(ThreadContext::new());
}

/// 在本线程的 [`ThreadContext`] 上执行 `f`（cpp `getThreadContext()` 的惯用等价
/// 写法：交出引用改为交出借用期）。
///
/// cpp 镜像工件（`TimeTrace.h/cpp` 的 `getThreadContext`），sync-cpp 维护；保留
/// `pub`：`LUAU_TIMETRACE_ARGUMENT` 启用形态的下游展开 ABI，且 `tests/thread_context.rs`
/// 以外部测试身份验证其 TLS 语义。
///
/// 线程已进入退出阶段（TLS 已销毁）时静默跳过：所有调用点都在打点或 `Drop`
/// 路径上，此时 panic 只会让线程退出二次失败，cpp 在该情形下也不再产出事件。
pub fn with_thread_context(f: impl FnOnce(&mut ThreadContext)) {
  let _ = CONTEXT.try_with(|cell| f(&mut cell.borrow_mut()));
}
