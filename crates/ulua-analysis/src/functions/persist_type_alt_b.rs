use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack, get_type_pack::get_type_pack_id,
    persist_type::persist as persist_type,
  },
  records::{
    generic_type_pack::GenericTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn persist(tp: TypePackId) {
  // SAFETY: tp 指向类型 arena 中的 TypePack；C++ TypePack::persistent 原地置位
  unsafe {
    if (*tp).persistent {
      return;
    }

    (*as_mutable_type_pack(tp)).persistent = true;
  }

  if let Some(p) = get_type_pack_id::<TypePack>(tp) {
    for &ty in &p.head {
      persist_type(ty);
    }
    if let Some(tail) = p.tail {
      unsafe { persist(tail) };
    }
    return;
  }

  if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
    persist_type(vtp.ty);
    return;
  }

  if get_type_pack_id::<GenericTypePack>(tp).is_some() {
    return;
  }

  if let Some(tfitp) = get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) {
    for &ty in tfitp.type_arguments.iter() {
      persist_type(ty);
    }

    for &tp in tfitp.pack_arguments.iter() {
      unsafe { persist(tp) };
    }
    return;
  }

  LUAU_ASSERT!(
    false /* "TypePackId is not supported in a persist call" */
  );
}
