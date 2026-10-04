use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int createGeneric(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:572`）。
use crate::{
  functions::{alloc_type_user_data::alloc_type_user_data, throw_type_error::throw_type_error},
  records::type_function_generic_type::TypeFunctionGenericType,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};
pub(crate) fn create_generic(l: &mut LuaState) -> i32 {
  // cpp `const char* name = luaL_checkstring(L, 1)` 后紧接 `luaL_optboolean`：先把名字
  // 落成拥有值（alloc_type_user_data 组装 variant 时同样拷贝），借用窗不跨过后续栈操作。
  let name = l.check_str(1).to_owned();
  let is_pack = l.opt_boolean(2, false);

  if name.is_empty() {
    throw_type_error(
      l,
      format_args!("types.generic: generic name cannot be empty"),
    );
  }

  let generic_type = TypeFunctionGenericType {
    is_named: true,
    is_pack,
    name,
  };

  alloc_type_user_data(l, TypeFunctionTypeVariant::Generic(generic_type), false);

  1
}
