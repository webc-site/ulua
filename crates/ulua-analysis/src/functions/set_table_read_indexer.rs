use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int setTableReadIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1114`）。
use crate::functions::throw_type_error::{SEPARATE_RW_INDEXER_MSG, throw_type_error};
pub(crate) fn set_table_read_indexer(l: &mut LuaState) -> i32 {
  throw_type_error(
    l,
    format_args!("type.setreadindexer: {SEPARATE_RW_INDEXER_MSG}"),
  )
}
