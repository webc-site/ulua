use crate::{
  records::{generic_type::GenericType, skip_cache_for_type::SkipCacheForType},
  type_aliases::type_id::TypeId,
};

impl SkipCacheForType {
  pub fn visit_type_id_generic_type(&mut self, _ty: TypeId, _gt: &GenericType) -> bool {
    self.result = true;
    false
  }
}
