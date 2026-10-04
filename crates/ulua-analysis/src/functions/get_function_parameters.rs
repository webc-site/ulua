use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int getFunctionParameters(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1384`）。
use crate::functions::type_function_runtime_entries::get_function_pack;
pub(crate) fn get_function_parameters(l: &mut LuaState) -> i32 {
  get_function_pack(l, "type.parameters", true)
}
