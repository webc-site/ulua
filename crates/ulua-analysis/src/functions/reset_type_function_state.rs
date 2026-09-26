use ulua_vm::{
  functions::{lua_call::lua_call, lua_getfield::lua_getfield, lua_pushnumber::lua_pushnumber},
  macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop},
  records::lua_state,
};

use crate::{
  functions::lua_names::{GLOBAL_MATH, GLOBAL_RANDOMSEED},
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是已调用过 `setTypeFunctionEnvironment`、主线程已挂载 `TypeFunctionRuntime` 的
/// `lua_State*`；本函数通过一次 Lua 调用重置运行期状态，要求 VM 栈处于单线程可调用状态。对应 C++
/// `void resetTypeFunctionState(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2149`）。
pub unsafe fn reset_type_function_state(l: *mut LuaState) {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_getglobal(vm_l, GLOBAL_MATH.as_ptr().cast());
    lua_getfield(vm_l, -1, GLOBAL_RANDOMSEED.as_ptr().cast());
    lua_pushnumber(vm_l, 0.0);
    lua_call(vm_l, 1, 0);
    lua_pop(vm_l, 1);
  }
}
