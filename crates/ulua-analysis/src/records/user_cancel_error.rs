use crate::records::internal_compiler_error::InternalCompilerError;

/// 用户取消错误：文案转发 `base.message`，与旧手写 `Display` 逐字一致。`base` 不标注
/// `#[source]`，以保持旧 `impl Error` 的 `source()` 恒为 `None` 的行为。
#[derive(Debug, Clone, thiserror::Error)]
#[error("{}", .base.message)]
pub struct UserCancelError {
  pub base: InternalCompilerError,
}
