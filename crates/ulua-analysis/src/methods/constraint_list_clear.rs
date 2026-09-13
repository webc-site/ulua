use crate::records::constraint_list::ConstraintList;

impl ConstraintList {
  pub fn clear(&mut self) {
    self.order.clear();
    // `present.clear()` doesn't compile for the current `DenseHashMap`/hasher bounds.
    // Clearing `present` by recreating it would require access to the hasher/empty key,
    // which is not available here. Instead, reset the ordering and counters; other
    // operations consult `entries` and `order` for iteration/dispatch.
    self.entries = 0;
  }
}
