extern crate alloc;

use alloc::string::{String, ToString};

use crate::records::constraint::Constraint;

pub fn to_pointer_id_not_null_constraint(ptr: *const Constraint) -> String {
  (ptr as usize).to_string()
}
