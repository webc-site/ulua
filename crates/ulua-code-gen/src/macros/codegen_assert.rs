//! Node: `cxx:Macro:Luau.CodeGen:CodeGen/include/Luau/CodeGenCommon.h:CODEGEN_ASSERT`
//! (hand-fixed: the original translation passed `&str` where the handler
//! takes `*const c_char` and could never have expanded; mirrors LUAU_ASSERT!)

// 宏体内 unsafe 为 FFI 语义必需:供安全上下文中的调用点使用;
// 在 unsafe 上下文中展开时会报 unused_unsafe(多展开点重复计数),属已知误报。
use core::ffi::c_char;
#[inline(never)]
#[cold]
pub fn codegen_assert_fail(expr_with_nul: &str, file_with_nul: &str, line: i32) -> bool {
  let expr = if expr_with_nul.ends_with('\0') {
    expr_with_nul.as_ptr() as *const c_char
  } else {
    c"<invalid expr>".as_ptr()
  };
  let file = if file_with_nul.ends_with('\0') {
    file_with_nul.as_ptr() as *const c_char
  } else {
    c"<invalid file>".as_ptr()
  };
  unsafe { ulua_common::assert_call_handler(expr, file, line, c"unknown".as_ptr()) != 0 }
}

#[macro_export]
macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    if !($expr) {
      if $crate::macros::codegen_assert::codegen_assert_fail(
        concat!(stringify!($expr), "\0"),
        concat!(file!(), "\0"),
        line!() as i32,
      ) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }
  };
  ($expr:expr, $msg:expr) => {
    if !($expr) {
      if $crate::macros::codegen_assert::codegen_assert_fail(
        concat!(stringify!($expr), " : ", stringify!($msg), "\0"),
        concat!(file!(), "\0"),
        line!() as i32,
      ) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }
  };
}

pub use CODEGEN_ASSERT;
