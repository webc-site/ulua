use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int readTableProp(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1009`）。
use crate::functions::type_function_runtime_entries::get_table_prop;
pub(crate) fn read_table_prop(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `read = true` 选择读取 prop 的 `read_ty`。
  unsafe { get_table_prop(l, "type.readproperty", true) }
}
