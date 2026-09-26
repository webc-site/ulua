use crate::{
  functions::{follow_type, follow_type_pack},
  records::type_cacher::TypeCacher,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeCacher {
  pub fn is_uncacheable_type_id(&self, ty: TypeId) -> bool {
    self.uncacheable.contains(&follow_type::follow(ty))
  }

  pub(crate) fn is_uncacheable_type_pack_id(&self, tp: TypePackId) -> bool {
    self
      .uncacheable_packs
      .contains(&follow_type_pack::follow(tp))
  }
}
