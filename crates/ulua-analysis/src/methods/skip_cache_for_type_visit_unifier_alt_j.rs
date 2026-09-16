use crate::{
  records::skip_cache_for_type::SkipCacheForType,
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
};

impl SkipCacheForType {
  pub fn visit_type_pack_id_bound_type_pack(
    &mut self,
    _tp: TypePackId,
    _btp: &BoundTypePack,
  ) -> bool {
    self.result = true;
    false
  }
}
