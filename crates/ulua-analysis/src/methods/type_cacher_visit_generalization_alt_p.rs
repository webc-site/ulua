use crate::{
  functions::follow_type::follow_type_id,
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  records::{intersection_type::IntersectionType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_intersection_type(&mut self, ty: TypeId, it: &IntersectionType) -> bool {
    if self.is_uncacheable_type_id(ty) || self.is_cached(ty) {
      return false;
    }

    let mut uncacheable = false;
    for &part in &it.parts {
      let followed = follow_type_id(part);
      cacher_traverse_type_id(self, followed);
      uncacheable |= self.is_uncacheable_type_id(followed);
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }
}
