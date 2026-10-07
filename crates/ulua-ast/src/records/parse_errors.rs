use alloc::{string::String, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::parse_error::ParseError;

/// 解析错误集合：`Display`/`Error` 由 thiserror derive 生成，文案即 `message`。
/// 旧 `Display` 为 `f.write_str(&self.message)`，对 `String` 与本写法逐字节相同。
#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct ParseErrors {
  pub(crate) errors: Vec<ParseError>,
  pub(crate) message: String,
}

impl ParseErrors {
  pub fn new(errors: Vec<ParseError>) -> Self {
    LUAU_ASSERT!(!errors.is_empty());

    let message = if errors.len() == 1 {
      errors[0].what().to_string()
    } else {
      alloc::format!("{} parse errors", errors.len())
    };

    Self { errors, message }
  }

  pub fn get_errors(&self) -> &Vec<ParseError> {
    &self.errors
  }

  pub fn what(&self) -> &str {
    &self.message
  }
}
