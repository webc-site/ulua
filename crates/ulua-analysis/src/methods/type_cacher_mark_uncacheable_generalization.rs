use crate::{
  functions::{follow_type, follow_type_pack},
  records::type_cacher::TypeCacher,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeCacher {
  pub fn mark_uncacheable_type_id(&mut self, ty: TypeId) {
    let followed = follow_type::follow(ty);
    self.uncacheable.insert(followed);
  }

  pub(crate) fn mark_uncacheable_type_pack_id(&mut self, tp: TypePackId) {
    let followed = follow_type_pack::follow(tp);
    self.uncacheable_packs.insert(followed);
  }
}
