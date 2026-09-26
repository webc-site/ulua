//! 对应 C++ `safeGetTable`（Repl.cpp:320）：沿 `__index` 元表链查找栈顶键，
//! 命中或达到遍历上限即停。
//!
//! 消费面：ulua-repl-cli 的 `complete_indexer`（本函数）与
//! `complete_partial_matches`（仅复用 [`MAX_TRAVERSAL_LIMIT`]）；ulua-cli-test
//! 的 `repl_complete_*` 用例经 REPL 补全入口间接覆盖本函数。

use ulua_vm::{
  functions::{
    lua_l_getmetafield::lua_l_getmetafield, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
    lua_rawget::lua_rawget, lua_remove::lua_remove, lua_replace::lua_replace,
  },
  macros::{lua_isnil::lua_isnil, lua_istable::lua_istable, lua_pop::lua_pop},
  records::lua_state::LuaState,
};

/// 对应 C++ `MaxTraversalLimit`（Repl.cpp），safe_get_table 与补全逻辑复用。
pub const MAX_TRAVERSAL_LIMIT: i32 = 50;

/// 元表 `__index` 字段名（NUL 结尾字节串）：本 crate 内沿元表链查找的
/// 两处消费点（safe_get_table / try_replace_top_with_index）共用，
/// 仅在对 `lua_l_getmetafield` 的收口点转 C 指针。
pub const META_INDEX_FIELD: &[u8] = b"__index\0";

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针；`table_index` 指向栈上的表，
/// 且待查找的键位于栈顶。
pub unsafe fn safe_get_table(l: *mut LuaState, table_index: i32) {
  // 下述每个小块只作用于 `l` 与本函数自己压入的栈槽，相对索引（`-1`/`-2`）的
  // 界内性由循环不变式（见各块注释）逐步支撑。
  //
  // 循环不变式：待搜索的表在 -1，键在 -2；退出时结果（值或 nil）在 -1。
  // Safety: `table_index` 是 `# Safety` 契约保证的有效栈索引，压入表副本建立不变式。
  unsafe { lua_pushvalue(l, table_index) }; // 复制表

  let mut loop_count: i32 = 0;
  loop {
    // Safety: 按不变式表在 -1、键在 -2；pushvalue 复制键后顶是副本、-2 是表；
    // rawget 弹副本查表并把结果压回顶（此时结果 -1、表 -2、原键 -3）。
    unsafe {
      lua_pushvalue(l, -2); // 复制键
      lua_rawget(l, -2); // 尝试查找键
    }

    // Safety: -1 是上块压入的查找结果。
    if (unsafe { !lua_isnil!(l, -1) }) || loop_count >= MAX_TRAVERSAL_LIMIT {
      break;
    }

    // Safety: 弹出 nil 结果，栈回到不变式形态（表在 -1、键在 -2）。
    unsafe { lua_pop(l, 1) }; // 弹出 nil 结果
    // Safety: -1 是被搜索的表；查其 `__index` 元字段，命中则压入字段值。
    if unsafe { lua_l_getmetafield(l, -1, META_INDEX_FIELD.as_ptr().cast()) } == 0 {
      // Safety: getmetafield 返 0 时未压栈；补 nil 作为查找结果后退出。
      unsafe { lua_pushnil(l) };
      break;
    } else if lua_istable!(l, -1) {
      // （宏自带 unsafe 圈与展开点契约：-1 是刚压入的 __index 字段）
      // 用 __index 表替换当前被搜索的表
      // Safety: 栈顶是 __index 表、-2 是当前表；replace 弹顶写入 -2 槽位，
      // 新表回到 -1，不变式保持。
      unsafe { lua_replace(l, -2) };
    } else {
      // Safety: 弹出非表字段值，补 nil 作为查找结果后退出。
      unsafe {
        lua_pop(l, 1); // 弹出值
        lua_pushnil(l);
      }
      break;
    }

    loop_count += 1;
  }

  // Safety: 此刻栈（自底向上）为 原始键、表、结果；-2 即表。
  unsafe { lua_remove(l, -2) }; // 移除表
  // Safety: 表已移除，-2 即原始键；弹后栈高回到调用前、只剩结果在顶。
  unsafe { lua_remove(l, -2) }; // 移除原始键
}
