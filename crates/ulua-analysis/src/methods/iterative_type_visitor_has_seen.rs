use core::ffi::c_void;

use crate::records::iterative_type_visitor::IterativeTypeVisitor;
impl IterativeTypeVisitor {
  pub fn iterative_type_visitor_has_seen(&mut self, tv: *const c_void) -> bool {
    if !self.visit_once {
      return false;
    }

    // C++ `bool isFresh = seen.insert(tv); return !isFresh;` — `Set::insert`
    // returns true when the element was newly inserted. The Rust `SeenSet`
    // (`DenseHashSet`) `insert` does not report freshness, so we probe
    // `contains` first to recover the same boolean.
    let is_fresh = !self.seen.contains(&tv);
    self.seen.insert(tv);
    !is_fresh
  }
}
