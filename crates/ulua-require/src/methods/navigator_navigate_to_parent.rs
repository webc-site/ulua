use alloc::string::String;

use crate::{
  enums::navigate_result::NavigateResult,
  records::navigator::{Error, Navigator},
};

impl Navigator<'_> {
  pub fn navigate_to_parent(&mut self, previous_component: Option<&str>) -> Error {
    let result = self.navigation_context.to_parent();
    if result == NavigateResult::Success {
      return None;
    }

    let mut error_message = if let Some(previous_component) = previous_component {
      let mut message = String::from("could not get parent of component \"");
      message.push_str(previous_component);
      message.push('"');
      message
    } else {
      String::from("could not get parent of requiring context")
    };

    if result == NavigateResult::Ambiguous {
      error_message.push_str(" (ambiguous)");
    }
    Some(error_message)
  }
}
