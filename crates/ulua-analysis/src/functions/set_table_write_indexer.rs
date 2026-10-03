

/// 对应 C++ 原生 `static int setTableWriteIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1121`）。
use crate::{functions::throw_type_error::{SEPARATE_RW_INDEXER_MSG, throw_type_error}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn set_table_write_indexer(l: &mut LuaState) -> i32 {
  // Safety: `l` 的有效性由本函数前置条件给出；消息无占位符，抛错后不返回。
  unsafe {
    throw_type_error(l,
      format_args!("type.setwriteindexer: {SEPARATE_RW_INDEXER_MSG}"),
    )
  }
}
