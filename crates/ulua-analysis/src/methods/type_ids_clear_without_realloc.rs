use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn clear_without_realloc(&mut self) {
    // Clear the logical contents without forcing underlying allocations to be released.
    // - DenseHashMap: clear() preserves capacity; no threshold parameter needed.
    // - order: clear the vector (capacity is preserved).
    // - hash: reset.
    self.order.clear();
    self.types.clear();
    self.hash = 0;
  }
}
