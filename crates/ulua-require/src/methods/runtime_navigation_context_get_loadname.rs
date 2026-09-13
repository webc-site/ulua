use alloc::string::String;

use crate::records::runtime_navigation_context::{
  INITIAL_IDENTIFIER_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn get_loadname(&self) -> Option<String> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_loadname?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }
}
