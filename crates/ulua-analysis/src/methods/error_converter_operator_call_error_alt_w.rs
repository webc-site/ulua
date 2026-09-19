use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, extra_information::ExtraInformation};

impl ErrorConverter {
  pub fn operator_call_23(&self, e: &ExtraInformation) -> String {
    String::from(e.message())
  }
}
