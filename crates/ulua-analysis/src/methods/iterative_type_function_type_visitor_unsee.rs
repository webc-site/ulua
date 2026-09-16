use core::ffi::c_void;

use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
impl IterativeTypeFunctionTypeVisitor {
  pub fn unsee(&mut self, _tv: *const c_void) {
    if !self.visit_once {
      // C++: `seen.erase(tv);`
      //
      // The C++ `SeenSet` is `Luau::Set` (DenseHashMap-backed, supports
      // erasure); here `SeenSet` is aliased to `DenseHashSet`, which has
      // no `erase`. This is sound regardless: `hasSeen` only inserts into
      // `seen` when `visitOnce` is true (it early-returns `false` before
      // any insert when `!visitOnce`), so in the `!visitOnce` branch the
      // set is always empty and the erase has nothing to remove. The
      // removal is therefore a no-op here, matching C++ behaviour.
    }
  }
}
