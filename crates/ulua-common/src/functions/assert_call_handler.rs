use core::ffi::{CStr, c_char};

use crate::{functions::assert_handler::assert_handler, macros::luau_noinline::LUAU_NOINLINE};

LUAU_NOINLINE! {
    /// # Safety
    /// `expression`、`file` 和 `function` 必须是有效的以 nul 结尾的 C 字符串或 null。
    pub unsafe fn assert_call_handler(
        expression: *const c_char,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> i32 {
        let handler_ptr = assert_handler();
        if let Some(handler) = *handler_ptr {
            unsafe {
                return handler(expression, file, line, function);
            }
        }

        // 无自定义处理器：在 LUAU_DEBUGBREAK 触发进程中断前打印断言信息
        // （与 C++ 默认的 assertCallHandler 行为一致，将消息输出至 stderr）。
        // 若没有此输出，失败将直接变为静默的 int 3，难以诊断。
        #[cfg(feature = "std")]
        unsafe {
            let expr = if expression.is_null() {
                "null"
            } else {
                CStr::from_ptr(expression).to_str().unwrap_or("null")
            };
            let f = if file.is_null() {
                "null"
            } else {
                CStr::from_ptr(file).to_str().unwrap_or("null")
            };
            eprintln!("LUAU_ASSERT failed: {} ({}:{})", expr, f, line);
        }

        1
    }
}

#[inline(never)]
#[cold]
pub fn assert_fail(expression_with_nul: &str, file_with_nul: &str, line: i32) {
  let expr = if expression_with_nul.ends_with('\0') {
    expression_with_nul.as_ptr() as *const c_char
  } else {
    c"<invalid expr>".as_ptr()
  };
  let file = if file_with_nul.ends_with('\0') {
    file_with_nul.as_ptr() as *const c_char
  } else {
    c"<invalid file>".as_ptr()
  };
  unsafe {
    assert_call_handler(expr, file, line, c"unknown".as_ptr());
  }
}
