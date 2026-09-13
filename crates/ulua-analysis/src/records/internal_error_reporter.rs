use alloc::string::String;
use core::fmt::{Debug, Formatter, Result};

use crate::type_aliases::frontend_callbacks::InternalErrorCallback;
#[derive(Clone, Default)]
pub struct InternalErrorReporter {
  pub on_internal_error: Option<InternalErrorCallback>,
  pub module_name: String,
}

impl Debug for InternalErrorReporter {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("InternalErrorReporter")
      .field(
        "on_internal_error",
        &self.on_internal_error.as_ref().map(|_| "..."),
      )
      .field("module_name", &self.module_name)
      .finish()
  }
}
