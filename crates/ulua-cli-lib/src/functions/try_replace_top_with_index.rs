//! Source: `CLI/src/Repl.cpp:306` (`tryReplaceTopWithIndex`)
//!
//! C++ 单点定义于 Repl.cpp, repl-cli 与 cli-test 共用一份实现。

use ulua_vm::{
  functions::{lua_l_getmetafield::lua_l_getmetafield, lua_remove::lua_remove},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must be a valid, active pointer to a `lua_State` with at least one element on its stack.
pub unsafe fn try_replace_top_with_index(l: *mut lua_State) -> bool {
  unsafe {
    if lua_l_getmetafield(l, -1, c"__index".as_ptr()) != 0 {
      // Remove the table leaving __index on the top of stack
      lua_remove(l, -2);
      true
    } else {
      false
    }
  }
}
