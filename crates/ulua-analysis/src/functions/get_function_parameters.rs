

/// 对应 C++ 原生 `static int getFunctionParameters(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1384`）。
use crate::{functions::type_function_runtime_entries::get_function_pack};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn get_function_parameters(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `params = true` 推 `arg_types` 整包。
  unsafe { get_function_pack(l, "type.parameters", true) }
}
