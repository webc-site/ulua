use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, occurs_check_failed::OccursCheckFailed};

impl ErrorConverter {
  pub fn operator_call_35(&self, _: &OccursCheckFailed) -> String {
    String::from("Type contains a self-recursive construct that cannot be resolved")
  }
}
