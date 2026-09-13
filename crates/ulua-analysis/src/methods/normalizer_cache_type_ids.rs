use crate::records::{normalizer::Normalizer, type_ids::TypeIds};

impl Normalizer {
  pub fn cache_type_ids(&mut self, tys: TypeIds) -> *const TypeIds {
    let tys_ptr = &tys as *const TypeIds;
    if let Some(found) = self.cached_type_ids.get(&tys_ptr) {
      return found.as_ref() as *const TypeIds;
    }

    let uniq = Box::new(tys);
    let result = uniq.as_ref() as *const TypeIds;
    self.cached_type_ids.insert(result, uniq);
    result
  }
}
