//! cpp 各 CLI 的 `static int assertionHandler(const char* expr, const char* file, int line, const char* function)`:
//! 打印到 stdout 并返回 1。各 CLI 的本地副本收敛于此。
use core::ffi::c_char;

use ulua_common::functions::{assert_handler::set_assert_handler, c_str::cstr_cow};

/// `AssertHandler` 回调实现, 签名匹配 `ulua_common::type_aliases::assert_handler::AssertHandler`
///
/// # Safety
/// 调用方 (C 运行时契约) 保证 `expr`/`file`/`_function` 为有效 NUL 结尾 C 字符串
pub unsafe extern "C-unwind" fn assertion_handler(
  expr: *const c_char,
  file: *const c_char,
  line: i32,
  _function: *const c_char,
) -> i32 {
  // Safety: 调用方 (C 运行时契约) 保证指针为有效 NUL 结尾 C 字符串
  let (file, expr) = unsafe { (cstr_cow(file), cstr_cow(expr)) };
  println!("{}({}): ASSERTION FAILED: {}", file, line, expr);
  1
}

/// 注册共享断言处理器, 等价 cpp `Luau::assertHandler() = assertionHandler;`
pub fn install_assertion_handler() {
  set_assert_handler(Some(assertion_handler));
}
