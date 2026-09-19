use crate::{
  functions::follow_type::follow_type_id,
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  records::{negation_type::NegationType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_negation_type(&mut self, ty: TypeId, nt: &NegationType) -> bool {
    if !self.is_cached(ty) && !self.is_uncacheable_type_id(ty) {
      let followed = follow_type_id(nt.ty);
      cacher_traverse_type_id(self, followed);

      if self.is_uncacheable_type_id(followed) {
        self.mark_uncacheable_type_id(ty);
      } else {
        self.cache(ty);
      }
    }
    false
  }
}
