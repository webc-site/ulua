use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{free_type_pack::FreeTypePack, reference_count_initializer::ReferenceCountInitializer},
  type_aliases::type_pack_id::TypePackId,
};

impl ReferenceCountInitializer {
  /// C++ `bool ReferenceCountInitializer::visit(TypePackId tp, const FreeTypePack&)`
  /// (Constraint.cpp:74-82).
  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    tp: TypePackId,
    _free_type_pack: &FreeTypePack,
  ) -> bool {
    if FFlag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(!self.mutated_type_packs.is_null());
      unsafe {
        (*self.mutated_type_packs).insert(tp);
      }
    }
    true
  }
}
