

/// # Safety
/// `l` 必须是已调用过 `setTypeFunctionEnvironment`、主线程已挂载 `TypeFunctionRuntime` 的
/// `lua_State*`；本函数通过一次 Lua 调用重置运行期状态，要求 VM 栈处于单线程可调用状态。对应 C++
/// `void resetTypeFunctionState(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2149`）。
use crate::{functions::lua_names::{GLOBAL_MATH, GLOBAL_RANDOMSEED}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) unsafe fn reset_type_function_state(l: &mut LuaState) {
  unsafe {
    l.get_global_bytes(GLOBAL_MATH);
    l.get_field_bytes(-1, GLOBAL_RANDOMSEED);
    l.push_number(0.0);
    l.call(1, 0);
    l.pop(1);
  }
}
