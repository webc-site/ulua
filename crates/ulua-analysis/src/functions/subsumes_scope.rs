use crate::{functions::subsumes_strict::subsumes_strict, records::scope::Scope};

pub fn subsumes(left: *mut Scope, right: *mut Scope) -> bool {
  if left.is_null() || right.is_null() {
    return false;
  }

  left == right || subsumes_strict(left, right)
}
