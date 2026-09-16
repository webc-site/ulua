use ulua_common::FInt;

use crate::records::normalizer::Normalizer;

impl Normalizer {
  pub fn within_resource_limits(&mut self) -> bool {
    // If cache is too large, clear it
    if FInt::LuauNormalizeCacheLimit.get() > 0 {
      let cache_usage = self.cached_normals.len()
        + self.cached_intersections.len()
        + self.cached_unions.len()
        + self.cached_type_ids.len()
        + self.cached_is_inhabited.size()
        + self.cached_is_inhabited_intersection.size();
      if cache_usage > FInt::LuauNormalizeCacheLimit.get() as usize {
        self.clear_caches();
        return false;
      }
    }

    // Check the recursion count
    !(unsafe { (*self.shared_state).counters.recursion_limit } > 0
      && unsafe { (*self.shared_state).counters.recursion_limit }
        < unsafe { (*self.shared_state).counters.recursion_count })
  }
}
