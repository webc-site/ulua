use crate::{
  records::{free_type_pack::FreeTypePack, skip_cache_for_type::SkipCacheForType},
  type_aliases::type_pack_id::TypePackId,
};

impl SkipCacheForType {
  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    _tp: TypePackId,
    _ftp: &FreeTypePack,
  ) -> bool {
    self.result = true;
    false
  }
}
