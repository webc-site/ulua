

/// 对应 C++ 原生 `static int getReadParent(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1494`）。
use crate::{functions::type_function_runtime_entries::get_parent};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn get_read_parent(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `read = true` 读取 `read_parent` 字段。
  unsafe { get_parent(l, true) }
}
