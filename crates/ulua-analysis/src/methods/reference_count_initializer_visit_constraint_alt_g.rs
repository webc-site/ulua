use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{
    blocked_type_pack::BlockedTypePack, reference_count_initializer::ReferenceCountInitializer,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl ReferenceCountInitializer {
  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _blocked_type_pack: &BlockedTypePack,
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
