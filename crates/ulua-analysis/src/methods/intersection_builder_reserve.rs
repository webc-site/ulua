use crate::records::intersection_builder::IntersectionBuilder;

impl IntersectionBuilder {
  pub fn reserve(&mut self, size: usize) {
    self.parts.reserve(size);
  }
}
