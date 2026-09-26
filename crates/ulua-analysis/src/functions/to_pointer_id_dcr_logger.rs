extern crate alloc;

use alloc::string::{String, ToString};

pub(crate) fn to_pointer_id<T>(ptr: *const T) -> String {
  (ptr as usize).to_string()
}

use crate::records::constraint::Constraint;

pub fn to_pointer_id_not_null_constraint(ptr: *const Constraint) -> String {
  (ptr as usize).to_string()
}
