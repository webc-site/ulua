use alloc::{fmt::format, string::String};
use core::fmt::Arguments;
use std::panic::panic_any;

use crate::records::location::Location;

/// 解析错误：`Display`/`Error` 由 thiserror derive 生成，文案即 `message`，与旧手写
/// `Display`（`write!(f, "{}", self.message)`）逐字一致。
#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
#[error("{message}")]
pub struct ParseError {
  pub(crate) location: Location,
  pub(crate) message: String,
}

impl ParseError {
  pub fn new(location: Location, message: String) -> Self {
    Self { location, message }
  }

  pub fn get_location(&self) -> &Location {
    &self.location
  }

  /// 只读消息文本。cpp 侧只有 `what()`（`Parser.cpp:114` 的 `const char*`），
  /// 这里让两个名字共享同一实现并统一返回 `&str`：`&String` 只是把字段类型
  /// 泄漏给调用方，徒增一层 deref。
  #[inline]
  pub fn get_message(&self) -> &str {
    self.what()
  }

  pub fn what(&self) -> &str {
    &self.message
  }

  pub fn raise(location: Location, args: Arguments<'_>) -> ! {
    let message = format(args);

    // cpp `throw ParseError(...)`。panic 载荷必须是 ParseError 对象本身：
    // `Parser::parse` 捕获 unwind 后经 `downcast_ref::<ParseError>()` 回收错误。
    // 若改用格式化 String panic（panic!("{}", ...)）该 downcast 会失败，
    // panic 将越过 parse 边界逃逸未被捕获（每个递归/错误上限用例会崩溃）。
    panic_any(ParseError::new(location, message));
  }
}
