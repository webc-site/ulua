use alloc::string::String;

use crate::records::increment_visitor::IncrementVisitor;
impl IncrementVisitor {
  pub fn operator_call(&self, v: &mut String) {
    v.push('1');
  }
}
