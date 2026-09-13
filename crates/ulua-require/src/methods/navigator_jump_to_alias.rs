use alloc::string::String;

use crate::{
  enums::navigate_result::NavigateResult,
  records::navigator::{Error, Navigator},
};

impl Navigator<'_> {
  pub fn jump_to_alias(&mut self, alias_path: &str) -> Error {
    let result = self.navigation_context.jump_to_alias(alias_path);
    if result == NavigateResult::Success {
      return None;
    }

    let mut error_message = String::from("could not jump to alias \"");
    error_message.push_str(alias_path);
    error_message.push('"');
    if result == NavigateResult::Ambiguous {
      error_message.push_str(" (ambiguous)");
    }
    Some(error_message)
  }
}
