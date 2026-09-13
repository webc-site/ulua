use crate::{
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_pack_id,
  records::type_cacher::TypeCacher,
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
};

impl TypeCacher {
  /// C++ `bool TypeCacher::visit(TypePackId tp, const BoundTypePack& btp)`
  /// (Generalization.cpp:631-637).
  pub fn visit_type_pack_id_bound_type_pack(
    &mut self,
    tp: TypePackId,
    btp: &BoundTypePack,
  ) -> bool {
    cacher_traverse_type_pack_id(self, btp.bound_to);
    if self.is_uncacheable_type_pack_id(btp.bound_to) {
      self.mark_uncacheable_type_pack_id(tp);
    }
    false
  }
}
