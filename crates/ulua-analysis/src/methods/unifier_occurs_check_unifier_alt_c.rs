use core::ptr::null_mut;

use crate::{
  records::{occurs_check_failed::OccursCheckFailed, type_pack_var::TypePackVar, unifier::Unifier},
  type_aliases::{
    type_error_data::TypeErrorData, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
impl Unifier {
  pub fn occurs_check_type_pack_id_type_pack_id_bool(
    &mut self,
    needle: TypePackId,
    haystack: TypePackId,
    _reversed: bool,
  ) -> bool {
    let shared_state = unsafe { &mut *self.shared_state };
    shared_state.temp_seen_tp.clear();

    let occurs = self.occurs_check_dense_hash_set_type_pack_id_type_pack_id_type_pack_id(
      &mut shared_state.temp_seen_tp,
      needle,
      haystack,
    );

    if occurs {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
      );
      // C++: log.replace(needle, BoundTypePack{builtinTypes->error_type_pack});
      // The Bound variant stores the bound-to pack id directly.
      let error_tp = unsafe { (*self.builtin_types).error_type_pack };
      let bound = TypePackVar {
        ty: TypePackVariant::Bound(error_tp),
        persistent: false,
        owning_arena: null_mut(),
      };
      self.log.replace_type_pack_id_type_pack_var(needle, bound);
    }

    occurs
  }
}
