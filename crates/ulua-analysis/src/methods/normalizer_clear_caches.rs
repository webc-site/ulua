use crate::records::normalizer::Normalizer;

impl Normalizer {
  pub fn clear_caches(&mut self) {
    self.cached_normals.clear();
    self.cached_intersections.clear();
    self.cached_unions.clear();
    self.cached_type_ids.clear();
  }
}
