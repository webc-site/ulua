use ulua_vm::records::lua_state;

use crate::{
  functions::throw_type_error::{SEPARATE_RW_INDEXER_MSG, throw_type_error},
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int setTableWriteIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1121`）。
pub unsafe fn set_table_write_indexer(l: *mut LuaState) -> i32 {
  // Safety: `l` 的有效性由本函数前置条件给出；消息无占位符，抛错后不返回。
  unsafe {
    throw_type_error(
      l as *mut lua_state::LuaState,
      format_args!("type.setwriteindexer: {SEPARATE_RW_INDEXER_MSG}"),
    )
  }
}
