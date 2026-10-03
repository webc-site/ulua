

/// 对应 C++ 原生 `static int createGeneric(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:572`）。
use crate::{functions::{alloc_type_user_data::alloc_type_user_data, throw_type_error::throw_type_error}, records::type_function_generic_type::TypeFunctionGenericType, type_aliases::{type_function_type_variant::TypeFunctionTypeVariant}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn create_generic(l: &mut LuaState) -> i32 {
  // Safety: l 是 Lua VM 调注册闭包时传入的存活 lua_State；luaL_checkstring 非字符串即抛错，
  // 成功返回 NUL 结尾、GC 在本次调用内保活的字符串，cstr_cow 读取合法；空名分支经
  // throw_type_error（内部收口格式串）终止；alloc_type_user_data 仅要求活状态，转发
  // 自同一约定。
  unsafe {
    // cpp `const char* name = luaL_checkstring(L, 1)` 后紧接 `luaL_optboolean`：先把名字
    // 落成拥有值（alloc_type_user_data 组装 variant 时同样拷贝），借用窗不跨过后续栈操作。
    let name = l.check_str(1).to_owned();
    let is_pack = l.opt_boolean(2, false);

    if name.is_empty() {
      throw_type_error(
        &mut *l,
        format_args!("types.generic: generic name cannot be empty"),
      );
    }

    let generic_type = TypeFunctionGenericType {
      is_named: true,
      is_pack,
      name,
    };

    alloc_type_user_data(
      &mut *l,
      TypeFunctionTypeVariant::Generic(generic_type),
      false,
    );

    1
  }
}
