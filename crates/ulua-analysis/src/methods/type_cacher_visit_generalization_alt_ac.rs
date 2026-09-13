use crate::{
  functions::{follow_type::follow_type_id, follow_type_pack::follow_type_pack_id},
  methods::type_cacher_visit_generalization_alt_i::{
    cacher_traverse_type_id, cacher_traverse_type_pack_id,
  },
  records::{type_cacher::TypeCacher, type_pack::TypePack},
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  pub fn visit_type_pack_id_type_pack(&mut self, tp: TypePackId, typ: &TypePack) -> bool {
    let mut uncacheable = false;
    for &ty in &typ.head {
      let followed = follow_type_id(ty);
      cacher_traverse_type_id(self, followed);
      uncacheable |= self.is_uncacheable_type_id(followed);
    }
    if let Some(tail) = typ.tail {
      let followed = unsafe { follow_type_pack_id(tail) };
      cacher_traverse_type_pack_id(self, followed);
      uncacheable |= self.is_uncacheable_type_pack_id(followed);
    }
    if uncacheable {
      self.mark_uncacheable_type_pack_id(tp);
    }
    false
  }
}
