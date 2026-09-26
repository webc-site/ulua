//! Source: `CLI/src/Repl.cpp:306` (`tryReplaceTopWithIndex`)
//!
//! C++ 单点定义于 Repl.cpp, repl-cli 与 cli-test 共用一份实现。

use ulua_vm::{
  functions::{lua_l_getmetafield::lua_l_getmetafield, lua_remove::lua_remove},
  records::lua_state::LuaState,
};

use crate::functions::safe_get_table::META_INDEX_FIELD;

/// # Safety
///
/// `l` must be a valid, active pointer to a `LuaState` with at least one element on its stack.
pub unsafe fn try_replace_top_with_index(l: *mut LuaState) -> bool {
  // Safety: `# Safety` 契约保证 `l` 活跃且栈非空，`-1` 即栈顶；`META_INDEX_FIELD`
  // 为静态 NUL 结尾串。命中时元方法被压栈，故 `-2` 仍是原表，可安全移除。
  let has_index = unsafe { lua_l_getmetafield(l, -1, META_INDEX_FIELD.as_ptr().cast()) != 0 };
  if !has_index {
    return false;
  }
  // Safety: 上一步把 __index 压到了栈顶，此时 `-2` 是它下面那张表。
  unsafe { lua_remove(l, -2) };
  true
}
