

/// 对应 C++ 原生 `static int setReadTableProp(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:913`）。
use crate::{functions::type_function_runtime_entries::set_table_prop_rw};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn set_read_table_prop(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `read = true` 改写 prop 的 `read_ty` 一侧。
  unsafe { set_table_prop_rw(l, "type.setreadproperty", true) }
}
