use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    get_type_pack::get_type_pack_id, queue_type_pack::queue_type_pack,
    try_unify_with_any::try_unify_with_any,
  },
  records::unifier::Unifier,
  type_aliases::{error_type_pack::ErrorTypePack, type_id::TypeId, type_pack_id::TypePackId},
};

impl Unifier {
  pub fn try_unify_with_any_type_pack_id_type_pack_id(
    &mut self,
    sub_ty: TypePackId,
    any_tp: TypePackId,
  ) {
    LUAU_ASSERT!(!get_type_pack_id::<ErrorTypePack>(any_tp).is_none());

    let any_ty: TypeId = unsafe { (*self.builtin_types).error_type };
    let mut queue: Vec<TypeId> = Vec::new();

    unsafe {
      let shared_state = &mut *self.shared_state;
      shared_state.temp_seen_ty.clear();
      shared_state.temp_seen_tp.clear();
      let seen_ty = &mut shared_state.temp_seen_ty as *mut _;
      let seen_tp = &mut shared_state.temp_seen_tp as *mut _;

      queue_type_pack(&mut queue, &mut *seen_tp, self, sub_ty, any_tp);
      try_unify_with_any(
        &mut queue,
        self,
        &mut *seen_ty,
        &mut *seen_tp,
        self.types,
        any_ty,
        any_tp,
      );
    }
  }
}
