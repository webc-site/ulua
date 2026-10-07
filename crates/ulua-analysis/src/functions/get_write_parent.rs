use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int getWriteParent(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1516`）。
use crate::functions::type_function_runtime_entries::get_parent;
pub(crate) fn get_write_parent(l: &mut LuaState) -> i32 {
  get_parent(l, false)
}
