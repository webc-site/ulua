use crate::{
  records::{blocked_type_pack::BlockedTypePack, skip_cache_for_type::SkipCacheForType},
  type_aliases::type_pack_id::TypePackId,
};

impl SkipCacheForType {
  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    _tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    self.result = true;
    false
  }
}
