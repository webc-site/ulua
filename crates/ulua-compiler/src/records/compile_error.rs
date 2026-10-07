use alloc::{fmt::format, string::String};
use core::fmt::Arguments;
use std::panic::panic_any;

use ulua_ast::records::location::Location;

extern crate alloc;

/// 编译期常量/跳转超限的通用报错文案（C++ 端字面量，勿改文案以免影响差异测试）。
pub(crate) const ERR_EXCEEDED_CONSTANT_LIMIT: &str =
  "Exceeded constant limit; simplify the code to compile";

/// 跳转距离超限的通用报错文案。
pub(crate) const ERR_EXCEEDED_JUMP_DISTANCE_LIMIT: &str =
  "Exceeded jump distance limit; simplify the code to compile";

/// 内联失败备注：递归/未建档调用不可内联（cpp 文案，两处发射点共用）。
pub(crate) const REMARK_INLINE_RECURSIVE: &str = "inlining failed: can't inline recursive calls";

/// 编译错误：`Display` / `Error` 由 thiserror derive 统一生成（文案即 `message`），
/// 收敛于工作区 thiserror 错误体系。
#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct CompileError {
  pub(crate) location: Location,
  pub(crate) message: String,
}

impl CompileError {
  pub(crate) fn new(location: Location, message: String) -> CompileError {
    CompileError { location, message }
  }

  pub fn get_location(&self) -> &Location {
    &self.location
  }

  /// C++ `static LUAU_NORETURN void raise(const Location&, const char* format, ...)`
  /// 调用方以 `format_args!(...)` 传入（对应 C++ 的变参约定）。
  pub fn raise(location: &Location, args: Arguments<'_>) -> ! {
    panic_any(CompileError::new(*location, format(args)))
  }

  /// 错误文案借用。cpp `std::exception::what()` 的 Rust 形态：直接借用 `message`，
  /// 不再物化 NUL 结尾副本（`Display` 亦由 thiserror derive 给出同一文案）。
  pub fn message(&self) -> &str {
    &self.message
  }
}
