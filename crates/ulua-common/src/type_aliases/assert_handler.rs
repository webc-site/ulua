//! cpp `Luau::AssertHandler`（Common/include/Luau/Common.h）的函数指针类型。
//!
//! 保留 `*const c_char` 与 `unsafe extern "C-unwind"`：这是宿主（含 C/C++ 侧）
//! 注入的回调 ABI 面，签名即契约，不能改成 `&str`/安全闭包。
//!
//! # Safety
//! 实现方必须：① 以 `extern "C-unwind"` 约定的调用约定可达（可跨 FFI 边界展开）；
//! ② 只在三个 `expression`/`file`/`function` 实参为 NUL 结尾 C 字符串或 null 时
//! 解引用它们，且不得在返回后保留这些指针；③ 返回非 0 表示"未接管，调用方可
//! 继续 debugbreak"，返回 0 表示"已接管"。

use core::ffi::c_char;

pub type AssertHandler = Option<
  unsafe extern "C-unwind" fn(
    expression: *const c_char,
    file: *const c_char,
    line: i32,
    function: *const c_char,
  ) -> i32,
>;
