use core::ptr::null_mut;

use crate::{functions::fresh_index::fresh_index, records::constraint::Constraint};

#[derive(Debug, Clone)]
pub struct BlockedTypePack {
  pub(crate) index: usize,
  pub(crate) owner: *mut Constraint,
}

impl BlockedTypePack {
  pub fn new() -> Self {
    Self {
      index: fresh_index() as usize,
      owner: null_mut(),
    }
  }
}

impl Default for BlockedTypePack {
  fn default() -> Self {
    Self::new()
  }
}
