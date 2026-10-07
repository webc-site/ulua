use core::{ffi::c_char, sync::atomic::Ordering};

use ulua_common::functions::assert_handler::{assert_handler, set_assert_handler};

use crate::records::assertion_catcher::{ASSERTION_CATCHER_TRIPPED, AssertionCatcher};

/// 宿主注入的 `LUAU_ASSERT` 处理器：只累计命中数，返回 0 表示「已接管」。
///
/// # Safety
/// 依 `AssertHandler` 契约：四个形参指向本次调用内有效的 NUL 结尾 C 串或 null；
/// 本实现不解引用它们，故无须额外前提。
unsafe extern "C-unwind" fn assertion_catcher_handler(
  _expression: *const c_char,
  _file: *const c_char,
  _line: i32,
  _function: *const c_char,
) -> i32 {
  ASSERTION_CATCHER_TRIPPED.fetch_add(1, Ordering::SeqCst);
  0
}

impl AssertionCatcher {
  pub fn new() -> Self {
    let previous = assert_handler();
    ASSERTION_CATCHER_TRIPPED.store(0, Ordering::SeqCst);
    set_assert_handler(Some(assertion_catcher_handler));

    Self { previous }
  }

  pub fn tripped() -> i32 {
    ASSERTION_CATCHER_TRIPPED.load(Ordering::SeqCst)
  }
}

impl Default for AssertionCatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for AssertionCatcher {
  fn drop(&mut self) {
    set_assert_handler(self.previous);
  }
}
