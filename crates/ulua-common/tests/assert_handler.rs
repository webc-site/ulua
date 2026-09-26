//! `LUAU_ASSERT!` 与 host 注入断言处理器的接管语义。
//!
//! 处理器是**进程级全局槽位**（`AtomicUsize`），这正是本文件必须是独立 test
//! binary 的理由：与同进程内其它测试共用该槽位时，互相会踩掉对方安装的处理器。

use core::{
  ffi::c_char,
  sync::atomic::{AtomicUsize, Ordering},
};

use ulua_common::{
  functions::{
    assert_call_handler::assert_fail,
    assert_handler::{assert_handler, set_assert_handler},
  },
  macros::{luau_assert::LUAU_ASSERT, luau_assertenabled::LUAU_ASSERTENABLED},
};

/// `LUAU_ASSERT` 展开点的数量，用于确认宏确实把失败交给了处理器。
static CALLED: AtomicUsize = AtomicUsize::new(0);

/// 模拟 host 侧"我已接管"的处理器：接管计数 +1 并返回 0（cpp 里 0 表示不需要
/// 再 `LUAU_DEBUGBREAK`）。测试进程若在抑制语义失效时走到断点分支会直接崩溃，
/// 因此本用例通过即证明 cpp `assertCallHandler(...) && DEBUGBREAK()` 的短路
/// 语义被保留。
unsafe extern "C-unwind" fn take_over(
  _expression: *const c_char,
  _file: *const c_char,
  _line: i32,
  _function: *const c_char,
) -> i32 {
  CALLED.fetch_add(1, Ordering::Relaxed);
  0
}

#[test]
fn handler_returning_zero_suppresses_debugbreak() {
  // release（`LUAU_ASSERT!` 整体不参与展开）下宏是空操作，计数恒为 0。
  if !LUAU_ASSERTENABLED {
    return;
  }
  set_assert_handler(Some(take_over));

  CALLED.store(0, Ordering::Relaxed);
  // 断言必然失败：宏只能经处理器退出（返回 0 即不触发断点）。
  LUAU_ASSERT!(false, "suppressed by host handler");
  assert_eq!(CALLED.load(Ordering::Relaxed), 1);

  // 直接调用 `assert_fail` 同样把处理器的 0 译成"不再断点"。
  assert!(!assert_fail("expr\0", "file\0", 1));
  assert_eq!(CALLED.load(Ordering::Relaxed), 2);

  // 成立的条件完全不触达处理器，与 cpp `!!(expr) || ...` 的短路一致。
  LUAU_ASSERT!(true);
  assert_eq!(CALLED.load(Ordering::Relaxed), 2);

  set_assert_handler(None);
  assert!(assert_handler().is_none());
}
