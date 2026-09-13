use crate::{
  records::skip_cache_for_type::SkipCacheForType,
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};

impl SkipCacheForType {
  pub fn visit_type_id_bound_type(&mut self, _ty: TypeId, _bt: &BoundType) -> bool {
    self.result = true;
    false
  }
}
