//! C++ `threadContextProvider()`（`Common/src/TimeTrace.cpp`）：
//!
//! ```cpp
//! inline ThreadContextProvider& threadContextProvider()
//! {
//!     static ThreadContextProvider handler = nullptr;
//!     return handler;
//! }
//! ```
//!
//! 返回静态槽位引用供 host 读/写。Rust 以 `AtomicPtr` 承载同一槽位
//! （替代 `static mut` 的 C 风格写法），`null` 即 C++ 的默认 `nullptr`
//! —— 未设置 provider 时 `get_thread_context` 走线程局部实例。
use core::{
  ptr::null_mut,
  sync::atomic::{AtomicPtr, Ordering},
};

use crate::type_aliases::thread_context_provider::ThreadContextProvider;

static HANDLER: AtomicPtr<ThreadContextProvider> = AtomicPtr::new(null_mut());

/// 返回 provider 槽位。`load` 得 `null` 表示未设置；非空时以 `(*handler)()`
/// 取线程上下文指针。
pub fn thread_context_provider() -> &'static AtomicPtr<ThreadContextProvider> {
  &HANDLER
}

/// 当前 provider（`None` 表示未设置，等价 C++ 默认 `nullptr`）。
pub fn load_thread_context_provider() -> Option<ThreadContextProvider> {
  let p = HANDLER.load(Ordering::Relaxed);
  (!p.is_null()).then(|| unsafe { *p })
}
