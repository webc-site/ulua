use crate::records::union_builder::UnionBuilder;

impl UnionBuilder {
  pub fn reserve(&mut self, size: usize) {
    self.options.reserve(size);
  }
}
