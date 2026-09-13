use alloc::string::String;

use crate::records::runtime_navigation_context::{
  INITIAL_IDENTIFIER_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn get_alias(&self, alias: &str) -> Option<String> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_alias?;
    self.get_string_from_c_writer_with_input(writer, alias, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }
}
