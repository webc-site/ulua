use crate::{
  records::{blocked_type_pack::BlockedTypePack, type_cacher::TypeCacher},
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    self.mark_uncacheable_type_pack_id(tp);
    false
  }
}
