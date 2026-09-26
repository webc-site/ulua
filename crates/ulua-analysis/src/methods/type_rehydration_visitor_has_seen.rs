use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;

impl TypeRehydrationVisitor {
  /// C++ `bool hasSeen(const void* tv)`.
  pub fn has_seen(&mut self, tv: *const ()) -> bool {
    let ttv = tv as *mut ();
    if let Some(&count) = self.seen.get(&ttv)
      && count < self.count
    {
      return true;
    }

    self.seen.insert(ttv, self.count);
    false
  }
}
