use alloc::{string::String, vec::Vec};

use crate::records::to_string_span::ToStringSpan;

#[derive(Debug, Clone, Default)]
pub struct ToStringResult {
  pub name: String,
  /// Records which TypeId produced each substring of the output. Only recorded for named types
  pub type_spans: Vec<ToStringSpan>,
  pub invalid: bool,
  pub error: bool,
  pub cycle: bool,
  pub truncated: bool,
}

impl ToStringResult {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn type_spans(&self) -> &[ToStringSpan] {
    &self.type_spans
  }

  pub fn invalid(&self) -> bool {
    self.invalid
  }

  pub fn error(&self) -> bool {
    self.error
  }

  pub fn cycle(&self) -> bool {
    self.cycle
  }

  pub fn truncated(&self) -> bool {
    self.truncated
  }
}
