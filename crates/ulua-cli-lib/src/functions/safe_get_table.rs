//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：对应 C++ `safeGetTable`，沿
//! `__index` 元表链查找栈顶键，命中或达到遍历上限即停。两侧循环体逐分支
//! 相同（test 用 `loop`+break、repl 用等价 `while`），仅 `lua_State` 导入
//! 路径不同（`records` 与 `type_aliases` 指向同一类型）。

use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_l_getmetafield::lua_l_getmetafield, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
    lua_rawget::lua_rawget, lua_remove::lua_remove, lua_replace::lua_replace,
  },
  macros::{lua_isnil::lua_isnil, lua_istable::lua_istable, lua_pop::lua_pop},
  records::lua_state::lua_State,
};

/// 对应 C++ `MaxTraversalLimit`（Repl.cpp），safe_get_table 与补全逻辑复用。
pub const MAX_TRAVERSAL_LIMIT: c_int = 50;

/// # Safety
///
/// `l` 必须是有效、活跃的 `lua_State` 指针；`table_index` 指向栈上的表，
/// 且待查找的键位于栈顶。
pub unsafe fn safe_get_table(l: *mut lua_State, table_index: c_int) {
  unsafe {
    lua_pushvalue(l, table_index); // 复制表

    // 循环不变式：待搜索的表在 -1，键在 -2。
    let mut loop_count: c_int = 0;
    loop {
      lua_pushvalue(l, -2); // 复制键
      lua_rawget(l, -2); // 尝试查找键

      if !lua_isnil!(l, -1) || loop_count >= MAX_TRAVERSAL_LIMIT {
        break;
      }

      lua_pop(l, 1); // 弹出 nil 结果
      if lua_l_getmetafield(l, -1, c"__index".as_ptr()) == 0 {
        lua_pushnil(l);
        break;
      } else if lua_istable!(l, -1) {
        // 用 __index 表替换当前被搜索的表
        lua_replace(l, -2);
      } else {
        lua_pop(l, 1); // 弹出值
        lua_pushnil(l);
        break;
      }

      loop_count += 1;
    }

    lua_remove(l, -2); // 移除表
    lua_remove(l, -2); // 移除原始键
  }
}
