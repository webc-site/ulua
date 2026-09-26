use alloc::{fmt::format, string::String, vec::Vec};
use core::{ffi::c_char, fmt::Arguments};
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
  /// `message` 的 NUL 终止副本（剥内部 NUL + 补一个尾 NUL 的 `Vec<u8>`），
  /// 服务 C++ 形制的 [`CompileError::what`]：其返回 `*const c_char`，调用方以
  /// NUL 扫描定长读取。
  ///
  /// Rust `String` **不**带 NUL 终止符，若直接交出 `message.as_ptr()`，
  /// 读取方会越过 Buffer 尾部读进相邻内存——这类未定义行为会随分配器/负载
  /// 表现为偶发尾部乱码（即 issue #3 之后的跨平台故障）。改为构造期一次性
  /// 物化，指针在 `&self` 存活期内始终有效，对齐 C++ `std::string::c_str()`。
  /// 存储用 `Vec<u8>` 而非 `CString`：NUL 结尾由构造保证，不外泄 C 类型。
  pub(crate) c_message: Vec<u8>,
}

/// 由 `s` 构造 NUL 终止字节串（`Vec<u8>`：剥除任何内嵌 NUL 后补一个尾 NUL）。
/// 编译错误消息不含内嵌 NUL 字节；剥除是防御性的，保证构造（可能发生在栈
/// 展开途中）自身永不失败。与旧 `CString::new(s)` Ok 分支逐字节等价。
pub(crate) fn nul_terminated(s: &str) -> Vec<u8> {
  let stripped = s.replace('\0', "");
  let mut v = Vec::with_capacity(stripped.len() + 1);
  v.extend_from_slice(stripped.as_bytes());
  v.push(0);
  v
}

impl CompileError {
  /// 构造 `CompileError`，同时物化 NUL 终止的 `what()` 视图。
  pub(crate) fn new(location: Location, message: String) -> CompileError {
    let c_message = nul_terminated(&message);
    CompileError {
      location,
      message,
      c_message,
    }
  }

  pub fn get_location(&self) -> &Location {
    &self.location
  }

  /// C++ `static LUAU_NORETURN void raise(const Location&, const char* format, ...)`
  /// 调用方以 `format_args!(...)` 传入（对应 C++ 的变参约定）。
  pub fn raise(location: &Location, args: Arguments<'_>) -> ! {
    panic_any(CompileError::new(*location, format(args)))
  }

  pub fn what(&self) -> *const c_char {
    // 已 NUL 终止（见 `CompileError::c_message`）：Rust `String` 无 NUL 尾，
    // 直接给 `message.as_ptr()` 会让 `CStr::from_ptr` 越读。
    self.c_message.as_ptr().cast()
  }
}
