use core::ptr::null;

use crate::records::type_fun::TypeFun;
impl TypeFun {
  pub fn new() -> Self {
    Self {
      type_params: Vec::new(),
      type_pack_params: Vec::new(),
      r#type: null(),
      definition_location: None,
    }
  }
}

impl Default for TypeFun {
  fn default() -> Self {
    Self::new()
  }
}
