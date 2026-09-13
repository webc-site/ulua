use core::ffi::c_void;

use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;

impl TypeRehydrationVisitor {
  /// C++ `bool hasSeen(const void* tv)`.
  pub fn has_seen(&mut self, tv: *const c_void) -> bool {
    let ttv = tv as *mut c_void;
    if let Some(&count) = self.seen.get(&ttv)
      && count < self.count
    {
      return true;
    }

    self.seen.insert(ttv, self.count);
    false
  }
}
