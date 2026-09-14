use crate::{
  functions::{follow_type::follow_type_id, follow_type_pack::follow_type_pack_id},
  methods::type_cacher_visit_generalization_alt_i::{
    cacher_traverse_type_id, cacher_traverse_type_pack_id,
  },
  records::{type_cacher::TypeCacher, type_function_instance_type::TypeFunctionInstanceType},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    if self.is_cached(ty) || self.is_uncacheable_type_id(ty) {
      return false;
    }

    let mut uncacheable = false;
    for &arg in &tfit.type_arguments {
      let followed = follow_type_id(arg);
      cacher_traverse_type_id(self, followed);
      if self.is_uncacheable_type_id(followed) {
        uncacheable = true;
      }
    }

    for &pack in &tfit.pack_arguments {
      let followed = unsafe { follow_type_pack_id(pack) };
      cacher_traverse_type_pack_id(self, followed);
      if self.is_uncacheable_type_pack_id(followed) {
        uncacheable = true;
      }
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }
}
