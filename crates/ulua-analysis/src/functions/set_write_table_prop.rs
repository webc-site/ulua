use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int setWriteTableProp(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:961`）。
use crate::functions::type_function_runtime_entries::set_table_prop_rw;
pub(crate) fn set_write_table_prop(l: &mut LuaState) -> i32 {
  set_table_prop_rw(l, "type.setwriteproperty", false)
}
