use alloc::string::String;
#[derive(Debug, Clone)]
pub struct RuntimeErrorHandler {
  pub(crate) error_prefix: String,
  pub(crate) error_message: String,
}
