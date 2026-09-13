use ulua_vm::{
  functions::{lua_call::lua_call, lua_getfield::lua_getfield, lua_pushnumber::lua_pushnumber},
  macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop},
  records::lua_state,
};

use crate::type_aliases::lua_state::LuaState;
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn reset_type_function_state(l: *mut LuaState) {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_getglobal(vm_l, c"math".as_ptr());
    lua_getfield(vm_l, -1, c"randomseed".as_ptr());
    lua_pushnumber(vm_l, 0.0);
    lua_call(vm_l, 1, 0);
    lua_pop(vm_l, 1);
  }
}
