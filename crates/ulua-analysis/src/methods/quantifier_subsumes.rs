use alloc::sync::Arc;
use std::ptr::null_mut;

use crate::records::{quantifier::Quantifier, scope::Scope};
impl Quantifier {
  pub fn subsumes(&mut self, outer: *mut Scope, inner: *mut Scope) -> bool {
    let mut current = inner;
    while !current.is_null() {
      if current == outer {
        return true;
      }
      current = unsafe { (*current).parent.as_ref() }
        .map_or(null_mut(), |sp| Arc::as_ptr(sp) as *mut Scope);
    }
    false
  }
}
