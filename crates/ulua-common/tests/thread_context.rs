//! [`with_thread_context`] 的线程局部语义（TLS + 多线程）。
//!
//! 独立 test binary 的理由：本文件在多个线程上触碰进程内的 TLS 登记表
//! （`GlobalContext` 的 create_thread / release_thread），与 `src` 里其它打点单测
//! 同进程并行时互相干扰。
//!
//! 观测方式改用 pub API：`ThreadContext` 的事件缓冲是 crate 内部状态（`pub(crate)`
//! 字段），外部测试无权读写，故这里以「闭包拿到的实例地址」作为身份凭据——
//! cpp `thread_local ThreadContext context` 的两条语义（同线程同一实例、跨线程各自
//! 独立）正好就是身份问题。原测试的事件计数在 `src/lib.rs` 的
//! `timetrace_event_tests` 里已有等价覆盖；此处不再写入事件，也就无需在结尾
//! 清空缓冲（TLS 析构时不落盘）。

use std::{cell::Cell, thread};

use ulua_common::{
  functions::with_thread_context::with_thread_context, records::thread_context::ThreadContext,
};

/// 取本线程 TLS 上下文实例的地址（只读地址，不解引用）；TLS 已销毁时为 `None`。
fn context_addr() -> Option<usize> {
  let slot = Cell::new(None);
  with_thread_context(|context: &mut ThreadContext| {
    slot.set(Some(context as *const ThreadContext as usize));
  });
  slot.into_inner()
}

/// cpp `thread_local ThreadContext context` 的两条语义：同一线程看到的是同一个
/// 实例，不同线程各自独立。
#[test]
fn context_is_per_thread_singleton() {
  let main = context_addr().expect("主线程 TLS 上下文应可用");

  // 同一线程的第二次访问仍是同一实例：不是每次调用新建。
  assert_eq!(context_addr(), Some(main));

  // 另一个线程的上下文是全新的实例，看不到主线程的实例。
  let child = thread::spawn(context_addr)
    .join()
    .expect("子线程不应 panic")
    .expect("子线程 TLS 上下文应可用");
  assert_ne!(child, main);

  // 主线程的实例不受子线程影响。
  assert_eq!(context_addr(), Some(main));
}
