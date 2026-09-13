use crate::{
  records::{pending_expansion_type::PendingExpansionType, skip_cache_for_type::SkipCacheForType},
  type_aliases::type_id::TypeId,
};

impl SkipCacheForType {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _pet: &PendingExpansionType,
  ) -> bool {
    self.result = true;
    false
  }
}
