use alloc::{format, string::String};

use crate::records::{deprecated_api_used::DeprecatedApiUsed, error_converter::ErrorConverter};

impl ErrorConverter {
  pub fn operator_call_20(&self, e: &DeprecatedApiUsed) -> String {
    format!(
      "The property .{} is deprecated.  Use .{} instead.",
      e.symbol, e.use_instead
    )
  }
}
