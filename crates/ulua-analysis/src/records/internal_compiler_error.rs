use alloc::string::String;

use ulua_ast::records::location::Location;

/// 编译期内部错误：`Display`/`Error` 由 thiserror derive 生成，文案即 `message`，
/// 与旧手写 `Display`（`write!(f, "{}", self.message)`）逐字一致。
#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct InternalCompilerError {
  pub message: String,
  pub module_name: Option<String>,
  pub location: Option<Location>,
}

impl InternalCompilerError {
  pub fn new(message: String, module_name: Option<String>, location: Option<Location>) -> Self {
    Self {
      message,
      module_name,
      location,
    }
  }
}
