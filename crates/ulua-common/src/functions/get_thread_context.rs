//! Source: `Common/src/TimeTrace.cpp:253-261` (hand-ported)
//! C++:
//! ```cpp
//! ThreadContext& getThreadContext()
//! {
//!     if (auto provider = threadContextProvider())
//!         return provider();
//!     thread_local ThreadContext context;
//!     return context;
//! }
//! ```
use alloc::boxed::Box;

use crate::{
  functions::thread_context_provider::load_thread_context_provider,
  records::thread_context::ThreadContext,
};

thread_local! {
    // 线程局部 ThreadContext，Box 包装确保其堆地址在线程生命周期内稳定。
    static CONTEXT: Box<ThreadContext> = Box::new(ThreadContext::new());
}

/// cpp `getThreadContext()` 返回 `ThreadContext&`；Rust 无法在同一线程上
/// 存在多个嵌套借用者（`Scope` / `OptionalTailScope`）时给出唯一的 `&mut`，
/// 故以裸指针承载，与 `Scope::context` 字段同构。
///
/// # Safety
/// 返回的指针在其线程存活期内有效（TLS `Box` 的堆地址稳定，或由 host
/// provider 提供）；同一线程上的多个借用者共享同一实例，与 C++ 一致。
pub fn get_thread_context() -> *mut ThreadContext {
  // 检查可能实现自定义 TLS 的外部 provider（C++ `if (auto provider = threadContextProvider())`）
  if let Some(provider) = load_thread_context_provider() {
    let provided = provider();
    if !provided.is_null() {
      return provided;
    }
  }

  CONTEXT.with(|boxed| &**boxed as *const ThreadContext as *mut ThreadContext)
}
