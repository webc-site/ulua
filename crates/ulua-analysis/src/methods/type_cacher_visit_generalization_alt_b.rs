use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  records::{free_type::FreeType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  /// C++ `bool TypeCacher::visit(TypeId ty, const FreeType& ft)`
  /// (Generalization.cpp:285-299).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    // Free types are never cacheable.
    LUAU_ASSERT!(!self.is_cached(ty));

    if !self.is_uncacheable_type_id(ty) {
      cacher_traverse_type_id(self, ft.lower_bound);
      cacher_traverse_type_id(self, ft.upper_bound);

      self.mark_uncacheable_type_id(ty);
    }

    false
  }
}
