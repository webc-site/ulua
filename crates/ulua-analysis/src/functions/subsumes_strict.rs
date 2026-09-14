use std::sync::Arc;

use crate::records::scope::Scope;

pub fn subsumes_strict(left: *mut Scope, right: *mut Scope) -> bool {
  if left.is_null() || right.is_null() {
    return false;
  }

  let mut current = right;
  while !current.is_null() {
    let parent_opt = unsafe { (*current).parent.clone() };
    if let Some(ref parent_arc) = parent_opt {
      let parent_raw = Arc::as_ptr(parent_arc) as *mut Scope;
      if parent_raw == left {
        return true;
      }
      current = parent_raw;
    } else {
      break;
    }
  }

  false
}
