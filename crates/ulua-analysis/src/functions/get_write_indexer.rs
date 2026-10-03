use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int getWriteIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1740`）。
use crate::functions::type_function_runtime_entries::get_indexer;
pub(crate) fn get_write_indexer(l: &mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现。
  unsafe { get_indexer(l, "type.writeindexer") }
}
