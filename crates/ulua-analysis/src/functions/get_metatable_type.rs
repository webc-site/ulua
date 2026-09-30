use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, get_type, is_string::is_string},
  records::{
    builtin_types::BuiltinTypes, extern_type::ExternType, metatable_type::MetatableType,
    primitive_type::PrimitiveType,
  },
  type_aliases::type_id::TypeId,
};

pub fn get_metatable_type_id_not_null_builtin_types(
  type_: TypeId,
  builtin_types: &BuiltinTypes,
) -> Option<TypeId> {
  let type_ = follow_type::follow(type_);

  if let Some(mt_type) = get_type::get::<MetatableType>(type_) {
    return Some(mt_type.metatable);
  }

  if let Some(extern_type) = get_type::get::<ExternType>(type_) {
    return extern_type.metatable;
  }

  if is_string(type_) {
    // C++: string 的 metatable 在 BuiltinTypes 初始化时必已挂上（LUAU_ASSERT 必命中）
    // string_type 由 BuiltinTypes 构造期以 PrimitiveType 变体登记，下转必命中。
    let ptv = get_type::get::<PrimitiveType>(builtin_types.string_type)
      .expect("string_type 构造期即 PrimitiveType 变体，下转必命中");
    LUAU_ASSERT!(ptv.metatable.is_some());
    return ptv.metatable;
  }

  None
}
