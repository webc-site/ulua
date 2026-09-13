use core::ptr::null_mut;

use crate::records::local::Local;

impl Local {
  pub fn new() -> Self {
    Self {
      local: null_mut(),
      offset: 0,
    }
  }
}

pub fn parser_local_local() -> Local {
  Local::new()
}
