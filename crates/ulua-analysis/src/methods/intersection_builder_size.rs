use crate::records::intersection_builder::IntersectionBuilder;

impl IntersectionBuilder {
  pub fn size(&self) -> usize {
    self.parts.size()
  }
}
