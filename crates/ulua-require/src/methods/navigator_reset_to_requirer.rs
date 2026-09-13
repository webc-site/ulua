use alloc::string::String;

use crate::{
  enums::navigate_result::NavigateResult,
  records::navigator::{Error, Navigator},
};

impl Navigator<'_> {
  pub fn reset_to_requirer(&mut self) -> Error {
    let result = self.navigation_context.reset_to_requirer();
    if result == NavigateResult::Success {
      return None;
    }

    let mut error_message = String::from("could not reset to requiring context");
    if result == NavigateResult::Ambiguous {
      error_message.push_str(" (ambiguous)");
    }
    Some(error_message)
  }
}
