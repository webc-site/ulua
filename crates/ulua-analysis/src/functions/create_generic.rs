use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  functions::lua_l_optboolean::lua_l_optboolean, macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state,
};

use crate::{
  functions::{alloc_type_user_data::alloc_type_user_data, throw_type_error::throw_type_error},
  records::type_function_generic_type::TypeFunctionGenericType,
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int createGeneric(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:572`）。
pub unsafe fn create_generic(l: *mut LuaState) -> i32 {
  // Safety: l 是 Lua VM 调注册闭包时传入的存活 lua_State；luaL_checkstring 非字符串即抛错，
  // 成功返回 NUL 结尾、GC 在本次调用内保活的字符串，cstr_cow 读取合法；空名分支经
  // throw_type_error（内部收口格式串）终止；alloc_type_user_data 仅要求活状态，转发
  // 自同一约定。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let name_ptr = luaL_checkstring!(vm_l, 1);
    let is_pack = lua_l_optboolean(vm_l, 2, false);

    let name = cstr_cow(name_ptr);
    if name.is_empty() {
      throw_type_error(
        vm_l,
        format_args!("types.generic: generic name cannot be empty"),
      );
    }

    let generic_type = TypeFunctionGenericType {
      is_named: true,
      is_pack,
      name: name.into_owned(),
    };

    alloc_type_user_data(l, TypeFunctionTypeVariant::Generic(generic_type), false);

    1
  }
}
