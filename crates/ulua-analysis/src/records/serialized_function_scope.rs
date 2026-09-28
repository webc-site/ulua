use core::ptr::null_mut;

use crate::records::type_function_function_type::TypeFunctionFunctionType;
#[derive(Debug, Clone)]
pub struct SerializedFunctionScope {
  pub(crate) old_queue_size: usize,
  pub(crate) function: *mut TypeFunctionFunctionType,
}

impl Default for SerializedFunctionScope {
  fn default() -> Self {
    Self {
      old_queue_size: 0,
      function: null_mut(),
    }
  }
}
