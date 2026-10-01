//! 对应 cpp `CLI/src/Repl.cpp` 的 `static void completeIndexer`：从全局表出发
//! 按 `.` / `:` 逐级下钻，对最后一段前缀调用 `complete_partial_matches`。

use ulua_cli_lib::functions::{
  safe_get_table::safe_get_table, try_replace_top_with_index::try_replace_top_with_index,
};
use ulua_vm::{
  functions::lua_checkstack::lua_checkstack,
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_minstack::LUA_MINSTACK},
  records::lua_state::LuaState,
};

use crate::functions::{complete_partial_matches::complete_partial_matches, state_ref::state};

// DELIBERATE DEVIATION（review.md §9.3）：沿 `.`/`:` 逐级下钻全局表补全，全程在
// `*mut LuaState` 栈上 push/rawget/replace（ulua-vm c-API）；按 §2 收口为
// 「安全入口 + 入口单点物化 + 逐块 `// Safety:` 论证」形态。下钻字符串入参为借用
// `&str`，无 C 串透传。
//
// 前置条件（由调用方 get_completions 链成立）：`l` 必须是有效、活跃的
// `LuaState` 指针。
pub(crate) fn complete_indexer(
  l: *mut LuaState,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // Safety: l 为 REPL 补全链传入的存活状态（前置条件见上），经 `state` 门面物化后
  // 全走安全方法；unsafe 导出在各块内论证。
  let l = state(l);
  // Safety: `lua_checkstack` 为 unsafe 导出；先预留 LUA_MINSTACK 槽位再压入
  // LUA_GLOBALSINDEX 全局表起始搜索。
  unsafe { lua_checkstack(l, LUA_MINSTACK) };
  l.push_value(LUA_GLOBALSINDEX);

  let mut lookup: &str = edit_buffer;
  let mut complete_only_functions = false;

  loop {
    // cpp: find_first_of(".:")，npos 时整串即待补全前缀
    let Some(sep) = lookup.find(['.', ':']) else {
      // 栈顶为当前下钻到的表（首轮即全局表），complete_partial_matches
      // 仅读栈不改拓扑；prefix/lookup 借自本帧 &str。
      complete_partial_matches(
        l,
        complete_only_functions,
        edit_buffer,
        lookup,
        add_completion_callback,
      );
      break;
    };
    let prefix = &lookup[..sep];

    // find the key in the table
    // 安全方法压键：`push_bytes` 以字节切片全长为键（无 NUL 截断/补齐），
    // prefix 借自本帧 &str。
    l.push_bytes(prefix.as_bytes());
    // Safety: `safe_get_table` 为 unsafe fn，前提（-2 指向栈上表、键在栈顶）恰由
    // 本轮 push 建立；其/后续 lua_remove 保持每轮 push 后移除中间键、留下查询结果。
    unsafe { safe_get_table(l, -2) };
    l.remove(-2);

    // -1 为上一行 get_table 的结果槽；is_table/try_replace_top_with_index 只读栈顶
    // 并可在命中时以 __index/_index 表替换之（单槽改写，拓扑仍为 1 值）。
    // Safety: `try_replace_top_with_index` 为 unsafe fn，前提（栈顶为表）恰由上句成立。
    let descended = l.is_table(-1) || unsafe { try_replace_top_with_index(l) };
    if descended {
      // find(['.', ':']) 返回的 sep 必落在字符边界上
      complete_only_functions = lookup.as_bytes()[sep] == b':';
      lookup = &lookup[sep + 1..];
    } else {
      // Unable to search for keys, so stop searching
      break;
    }
  }

  // 与起始 lua_pushvalue 配平，收回栈顶表。
  l.pop(1);
}
