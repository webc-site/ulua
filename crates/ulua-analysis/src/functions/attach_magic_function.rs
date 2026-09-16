use alloc::sync::Arc;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{function_type::FunctionType, magic_function::MagicFunction},
  type_aliases::type_id::TypeId,
};

pub fn attach_magic_function(ty: TypeId, magic: Arc<MagicFunction>) {
  let Some(ftv) = get_mutable_type_id::<FunctionType>(ty) else {
    // C++ Type.cpp: LUAU_ASSERT(getMutable<FunctionType>(ty)) 后仅非空才赋值
    LUAU_ASSERT!(false);
    return;
  };
  ftv.magic = Some(magic);
}
