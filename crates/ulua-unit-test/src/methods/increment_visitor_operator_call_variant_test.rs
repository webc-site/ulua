use alloc::string::String;

use crate::records::increment_visitor::IncrementVisitor;

impl IncrementVisitor {
  pub fn operator_call(&self, v: &mut String) {
    v.push('1');
  }
}

impl IncrementVisitor {
  pub fn operator_call_mut(&self, v: &mut i32) {
    *v += 1;
  }
}
