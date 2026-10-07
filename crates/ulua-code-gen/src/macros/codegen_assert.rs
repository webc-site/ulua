//! Source: `CodeGen/include/Luau/CodeGenCommon.h:CODEGEN_ASSERT`（手工修正：
//! 原翻译在处理函数取 `*const c_char` 处传了 `&str`，永远无法展开；
//! 此处对齐 LUAU_ASSERT！）
//!
//! 失败上报直接复用 `ulua_common` 的 `assert_fail` 收口点（review.md §10：
//! 编译期 `concat!(…, "\0")` 产物按该处文档契约以 NUL 结尾字节串交给 C-ABI
//! 诊断入口，本 crate 不再自建 `c"..."` 字面量与逐串校验的副本）。
//!
//! See also: `functions/build_bytecode_blocks.rs` 与 `functions/get_scale_encoding.rs`
//! 各有一枚同名局部 `CODEGEN_ASSERT!`（标准 `assert!` 替身）——形似义异，勿合并。

/// 未知 IR 形态断言（48 处共用，消息与 C++ "Unsupported instruction form" 一致）。
/// 消息经 stringify! 只能取字面量，不能经 const 传递，故封装整条断言
#[inline(never)]
#[cold]
pub fn unsupported_instruction_form() {
  CODEGEN_ASSERT!(false, "Unsupported instruction form");
}

#[macro_export]
macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    if !($expr) {
      if ulua_common::functions::assert_call_handler::assert_fail(
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
      if ulua_common::functions::assert_call_handler::assert_fail(
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
