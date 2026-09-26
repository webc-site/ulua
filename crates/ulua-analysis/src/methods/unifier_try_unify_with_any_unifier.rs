use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    get_type, get_type_pack, queue_type_pack::queue_type_pack,
    try_unify_with_any::try_unify_with_any,
  },
  records::{
    any_type::AnyType, extern_type::ExternType, never_type::NeverType,
    primitive_type::PrimitiveType, unifier::Unifier, unknown_type::UnknownType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl Unifier {
  pub fn try_unify_with_any_type_id_type_id(&mut self, sub_ty: TypeId, any_ty: TypeId) {
    LUAU_ASSERT!(
      get_type::get::<AnyType>(any_ty).is_some()
        || get_type::get::<ErrorType>(any_ty).is_some()
        || get_type::get::<UnknownType>(any_ty).is_some()
        || get_type::get::<NeverType>(any_ty).is_some()
    );

    if get_type::get::<PrimitiveType>(sub_ty).is_some()
      || get_type::get::<AnyType>(sub_ty).is_some()
      || get_type::get::<ExternType>(sub_ty).is_some()
    {
      return;
    }

    let any_tp = self.types_mut().add_type_pack_t(VariadicTypePack {
      ty: any_ty,
      hidden: false,
    });
    let mut queue: Vec<TypeId> = alloc::vec![sub_ty];

    {
      let shared_state = self.shared_state_mut();
      shared_state.temp_seen_ty.clear();
      shared_state.temp_seen_tp.clear();
      let seen_ty = &mut shared_state.temp_seen_ty as *mut _;
      let seen_tp = &mut shared_state.temp_seen_tp as *mut _;
      // SAFETY: seen_ty/seen_tp 指向 shared_state 内存活字段，指针形态为
      // 规避 `self` 双重借用所需，回读前经裸指针解引用。
      unsafe {
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

  pub fn try_unify_with_any_type_pack_id_type_pack_id(
    &mut self,
    sub_ty: TypePackId,
    any_tp: TypePackId,
  ) {
    LUAU_ASSERT!(get_type_pack::get::<ErrorTypePack>(any_tp).is_some());

    let any_ty: TypeId = self.builtin_types_ref().error_type;
    let mut queue: Vec<TypeId> = Vec::new();

    {
      let shared_state = self.shared_state_mut();
      shared_state.temp_seen_ty.clear();
      shared_state.temp_seen_tp.clear();
      let seen_ty = &mut shared_state.temp_seen_ty as *mut _;
      let seen_tp = &mut shared_state.temp_seen_tp as *mut _;
      // SAFETY: seen_ty/seen_tp 指向 shared_state 内存活字段，指针形态为
      // 规避 `self` 双重借用所需，回读前经裸指针解引用。
      unsafe {
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
}
