//! 对应 C++ `safeGetTable`（Repl.cpp:320）：沿 `__index` 元表链查找栈顶键，
//! 命中或达到遍历上限即停。
//!
//! 消费面：ulua-repl-cli 的 `complete_indexer`（本函数）与
//! `complete_partial_matches`（仅复用 [`MAX_TRAVERSAL_LIMIT`]）；ulua-cli-test
//! 的 `repl_complete_*` 用例经 REPL 补全入口间接覆盖本函数。

use ulua_vm::{
  functions::{lua_l_getmetafield::lua_l_getmetafield_bytes, lua_rawget::lua_rawget},
  records::lua_state::LuaState,
};

use crate::functions::state_ref::state;

/// 对应 C++ `MaxTraversalLimit`（Repl.cpp），safe_get_table 与补全逻辑复用。
pub const MAX_TRAVERSAL_LIMIT: i32 = 50;

/// 元表 `__index` 字段名（字节切片，无尾部 NUL）：本 crate 内沿元表链查找的
/// 两处消费点（safe_get_table / try_replace_top_with_index）共用，均走
/// `lua_l_getmetafield_bytes` 的「切片全长即键」语义——携带尾部 NUL 会让
/// interned 键多一字节、恒查不中。
pub(crate) const META_INDEX_FIELD: &[u8] = b"__index";

// DELIBERATE DEVIATION（review.md §9.3）：全程在 ulua-vm c-API 栈上操作
// （pushvalue/rawget/pop/replace + `luaL_getmetafield_bytes`），`*mut LuaState`
// 解引用与相对索引是本函数赖以成立的 VM 边界固有形态，非纯 Rust 逻辑——本函数
// 即边界本体，故按 clippy `not_unsafe_ptr_arg_deref` 的判定留 `unsafe fn` +
// `# Safety` 契约；句柄解引用经 `state` 门面收敛为入口一次。
//
// review.md §2 的消除对象是「纯逻辑层的 pub unsafe fn」；此处 unsafe 并非逻辑层
// 装饰，而是「解引用调用方持有的 VM 状态裸指针」这一不可省略前提的类型化表达。
/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针；`table_index` 指向栈上的表，
/// 且待查找的键位于栈顶。
pub unsafe fn safe_get_table(l: *mut LuaState, table_index: i32) {
  // Safety: `# Safety` 契约保证 `l` 非空、活跃，经 `state` 门面物化后全走安全方法。
  let l = state(l);
  // 循环不变式：待搜索的表在 -1，键在 -2；退出时结果（值或 nil）在 -1。
  // 相对索引（`-1`/`-2`）的界内性由该不变式逐步支撑。
  l.push_value(table_index); // 复制表，建立不变式

  let mut loop_count: i32 = 0;
  loop {
    l.push_value(-2); // 复制键
    // 按不变式 `-2` 是表、顶是键副本，弹副本查表并把结果压回顶（此时结果 -1、
    // 表 -2、原键 -3）。
    lua_rawget(&mut *l, -2); // 尝试查找键

    if !l.is_nil(-1) || loop_count >= MAX_TRAVERSAL_LIMIT {
      break;
    }

    l.pop(1); // 弹出 nil 结果，栈回到不变式形态（表在 -1、键在 -2）
    // Safety: `lua_l_getmetafield_bytes` 为 ulua-vm unsafe 导出；`-1` 是被搜索的表，
    // 命中则压入 `__index` 字段值（切片全长即键，见 `META_INDEX_FIELD` 注）。
    if unsafe { lua_l_getmetafield_bytes(l, -1, META_INDEX_FIELD) } == 0 {
      // getmetafield 返 0 时未压栈；补 nil 作为查找结果后退出。
      l.push_nil();
      break;
    } else if l.is_table(-1) {
      // 用 __index 表替换当前被搜索的表：replace 弹顶写入 -2 槽位，新表回到 -1，
      // 不变式保持。
      l.replace(-2);
    } else {
      // 弹出非表字段值，补 nil 作为查找结果后退出。
      l.pop(1); // 弹出值
      l.push_nil();
      break;
    }

    loop_count += 1;
  }

  l.remove(-2); // 移除表：此刻栈自底向上为 原始键、表、结果，-2 即表
  l.remove(-2); // 移除原始键：弹后栈高回到调用前、只剩结果在顶
}
