use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int getFunctionReturns(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1423`）。
use crate::functions::type_function_runtime_entries::get_function_pack;
pub(crate) fn get_function_returns(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `params = false` 推 `ret_types` 整包。
  unsafe { get_function_pack(l, "type.returns", false) }
}
