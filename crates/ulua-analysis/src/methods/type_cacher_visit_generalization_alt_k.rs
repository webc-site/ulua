use crate::{
  functions::follow_type::follow_type_id,
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  records::{metatable_type::MetatableType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_metatable_type(&mut self, ty: TypeId, mtv: &MetatableType) -> bool {
    let tbl = follow_type_id(mtv.table());
    let mt = follow_type_id(mtv.metatable());
    cacher_traverse_type_id(self, tbl);
    cacher_traverse_type_id(self, mt);
    if self.is_uncacheable_type_id(tbl) || self.is_uncacheable_type_id(mt) {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }
    false
  }
}
