use alloc::string::String;

use crate::records::to_string_visitor::ToStringVisitor;
impl ToStringVisitor {
  pub fn operator_call(&self, v: &str) -> String {
    v.to_owned()
  }
}
