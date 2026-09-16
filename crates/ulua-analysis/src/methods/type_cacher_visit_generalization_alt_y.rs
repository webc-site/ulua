use crate::{
  functions::follow_type::follow_type_id,
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  records::{type_cacher::TypeCacher, variadic_type_pack::VariadicTypePack},
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  pub fn visit_type_pack_id_variadic_type_pack(
    &mut self,
    tp: TypePackId,
    vtp: &VariadicTypePack,
  ) -> bool {
    if self.is_uncacheable_type_pack_id(tp) {
      return false;
    }

    let followed = follow_type_id(vtp.ty);
    cacher_traverse_type_id(self, followed);

    if self.is_uncacheable_type_id(followed) {
      self.mark_uncacheable_type_pack_id(tp);
    }

    false
  }
}
