use alloc::string::String;

use crate::{
  enums::navigate_result::NavigateResult,
  records::navigator::{Error, Navigator},
};

impl Navigator<'_> {
  pub fn navigate_to_child(&mut self, component: &str) -> Error {
    let result = self.navigation_context.to_child(component);
    if result == NavigateResult::Success {
      return None;
    }

    let mut error_message = String::from("could not resolve child component \"");
    error_message.push_str(component);
    error_message.push('"');
    if result == NavigateResult::Ambiguous {
      error_message.push_str(" (ambiguous)");
    }
    Some(error_message)
  }
}
