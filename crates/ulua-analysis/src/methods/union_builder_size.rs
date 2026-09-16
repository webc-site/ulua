use crate::records::union_builder::UnionBuilder;

impl UnionBuilder {
  pub fn size(&self) -> usize {
    self.options.size()
  }
}
