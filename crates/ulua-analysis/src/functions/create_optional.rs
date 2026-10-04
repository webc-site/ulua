/// 对应 C++ 原生 `static int createOptional(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:625`）。
use alloc::vec::Vec;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    allocate_type_function_type::allocate_type_function_type,
    get_type_function_runtime::{get_type_function_runtime, get_type_function_type_id},
    get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_union_type::TypeFunctionUnionType,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_variant::TypeFunctionTypeVariant,
  },
};
pub(crate) fn create_optional(l: &mut LuaState) -> i32 {
  let runtime = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");
  lua_check_args!(l, != 1, "types.optional: expected 1 argument, but got {}");

  let argument: TypeFunctionTypeId = get_type_user_data(l, 1);

  let mut components: Vec<TypeFunctionTypeId> = Vec::new();

  if let Some(union_ty) = get_type_function_type_id::<TypeFunctionUnionType>(argument) {
    components.reserve(union_ty.components.len() + 1);
    components.extend(union_ty.components.iter().copied());
  } else {
    components.push(argument);
  }

  let nil_type = TypeFunctionPrimitiveType::new(Type::NilType);
  let nil_variant = TypeFunctionTypeVariant::Primitive(nil_type);
  let nil_id = allocate_type_function_type(runtime, nil_variant);
  components.push(nil_id);

  let union_type = TypeFunctionUnionType { components };
  let union_variant = TypeFunctionTypeVariant::Union(union_type);
  alloc_type_user_data(l, union_variant, false);

  1
}
