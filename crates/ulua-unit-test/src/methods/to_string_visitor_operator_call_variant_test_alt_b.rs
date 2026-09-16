use alloc::string::String;
use core::ffi::c_int;

use crate::records::to_string_visitor::ToStringVisitor;
impl ToStringVisitor {
  pub fn operator_call_mut(&self, v: c_int) -> String {
    alloc::format!("{}", v)
  }
}
