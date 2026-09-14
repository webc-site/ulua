//! cpp `Luau::assertHandler()`（Common/include/Luau/Common.h）：
//!
//! ```cpp
//! using AssertHandler = int (*)(const char* expression, const char* file, int line, const char* function);
//! AssertHandler& assertHandler();
//! ```
//!
//! C++ 以函数局部静态槽位承载；Rust 以 `AtomicUsize` 承载（替代 `static mut`
//! 的 C 风格写法），`0` 即 C++ 的默认 `nullptr`。
use core::{
  ffi::c_char,
  mem::transmute,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::type_aliases::assert_handler::AssertHandler;

static HANDLER: AtomicUsize = AtomicUsize::new(0);

/// 当前断言处理器（`None` 即 C++ 默认 `nullptr`）。
pub fn assert_handler() -> AssertHandler {
  let v = HANDLER.load(Ordering::Relaxed);
  if v == 0 {
    None
  } else {
    // SAFETY: `v` 来自 `set_assert_handler` 存储的函数指针，宽度一致。
    Some(unsafe {
      transmute::<
        usize,
        unsafe extern "C-unwind" fn(*const c_char, *const c_char, i32, *const c_char) -> i32,
      >(v)
    })
  }
}

/// `assertHandler() = handler` 对应的写入端。
pub fn set_assert_handler(handler: AssertHandler) {
  let v = handler.map(|f| f as usize).unwrap_or(0);
  HANDLER.store(v, Ordering::Relaxed);
}
