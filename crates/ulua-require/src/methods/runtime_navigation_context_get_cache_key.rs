use alloc::string::String;

use crate::records::runtime_navigation_context::{
  INITIAL_IDENTIFIER_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn get_cache_key(&self) -> Option<String> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_cache_key?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }
}
