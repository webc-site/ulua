

/// 对应 C++ 原生 `static int getWriteParent(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1516`）。
use crate::{functions::type_function_runtime_entries::get_parent};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn get_write_parent(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `read = false` 读取 `write_parent` 字段。
  unsafe { get_parent(l, false) }
}
