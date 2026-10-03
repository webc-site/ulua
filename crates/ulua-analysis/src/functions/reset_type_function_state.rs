//! 对应 C++ 原生 `void resetTypeFunctionState(lua_State* L)`
//! （`cpp/Analysis/src/TypeFunctionRuntime.cpp:2149`）。

use crate::functions::lua_names::{GLOBAL_MATH, GLOBAL_RANDOMSEED};
use ulua_vm::records::lua_state::LuaState;

/// 重置类型函数运行期的 Lua 侧状态：调用 `math.randomseed(0)` 后清掉返回值。
pub(crate) fn reset_type_function_state(l: &mut LuaState) {
  l.get_global_bytes(GLOBAL_MATH);
  l.get_field_bytes(-1, GLOBAL_RANDOMSEED);
  l.push_number(0.0);
  l.call(1, 0);
  l.pop(1);
}
