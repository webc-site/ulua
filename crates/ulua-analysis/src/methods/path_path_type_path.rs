use alloc::vec::Vec;

use crate::records::path::Path;
impl Path {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
    }
  }
}
