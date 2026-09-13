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
use core::cell::UnsafeCell;

use crate::{
  functions::thread_context_provider::thread_context_provider,
  records::thread_context::ThreadContext,
};

thread_local! {
    // 线程局部 ThreadContext，Box 包装确保其堆地址在线程生命周期内稳定。
    static CONTEXT: UnsafeCell<Box<ThreadContext>> =
        UnsafeCell::new(Box::new(ThreadContext::new()));
}

pub fn get_thread_context() -> &'static mut ThreadContext {
  // 检查可能实现自定义 TLS 的外部 provider
  let provider = *thread_context_provider();
  let provided = provider();
  if !provided.is_null() {
    return unsafe { &mut *provided };
  }

  CONTEXT.with(|cell| {
    // 安全性：Box<ThreadContext> 在线程生命周期内一直存活，其堆地址稳定。
    let boxed: &mut Box<ThreadContext> = unsafe { &mut *cell.get() };
    unsafe { &mut *(boxed.as_mut() as *mut ThreadContext) }
  })
}
