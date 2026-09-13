use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtraInformation {
  pub(crate) message: String,
}

impl ExtraInformation {
  pub const fn new(message: String) -> Self {
    Self { message }
  }
}

impl ExtraInformation {
  pub fn message(&self) -> &str {
    &self.message
  }
}
