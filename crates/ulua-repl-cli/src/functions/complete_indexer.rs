//! 对应 cpp `CLI/src/Repl.cpp` 的 `static void completeIndexer`：从全局表出发
//! 按 `.` / `:` 逐级下钻，对最后一段前缀调用 `complete_partial_matches`。

use ulua_cli_lib::functions::{
  safe_get_table::safe_get_table, try_replace_top_with_index::try_replace_top_with_index,
};
use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue,
    lua_remove::lua_remove,
  },
  macros::{
    lua_globalsindex::LUA_GLOBALSINDEX, lua_istable::lua_istable, lua_minstack::LUA_MINSTACK,
    lua_pop::lua_pop,
  },
  records::lua_state::LuaState,
};

use crate::functions::complete_partial_matches::complete_partial_matches;

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
pub(crate) unsafe fn complete_indexer(
  l: *mut LuaState,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // Safety: l 为 REPL 补全链传入的存活状态；先预留 LUA_MINSTACK 槽位再压入
  // LUA_GLOBALSINDEX 全局表起始搜索。
  unsafe {
    lua_checkstack(l, LUA_MINSTACK);
    lua_pushvalue(l, LUA_GLOBALSINDEX);
  }

  let mut lookup: &str = edit_buffer;
  let mut complete_only_functions = false;

  loop {
    // cpp: find_first_of(".:")，npos 时整串即待补全前缀
    let Some(sep) = lookup.find(['.', ':']) else {
      // Safety: 栈顶为当前下钻到的表（首轮即全局表），complete_partial_matches
      // 仅读栈不改拓扑；prefix/lookup 借自本帧 &str。
      unsafe {
        complete_partial_matches(
          l,
          complete_only_functions,
          edit_buffer,
          lookup,
          add_completion_callback,
        );
      }
      break;
    };
    let prefix = &lookup[..sep];

    // find the key in the table
    // Safety: prefix.as_ptr()/len 借自本地 &str 且 lua_pushlstring 当调用拷贝；
    // safe_get_table/lua_remove 保持每轮 push 后移除中间键、留下查询结果。
    unsafe {
      lua_pushlstring(l, prefix.as_ptr().cast(), prefix.len());
      safe_get_table(l, -2);
      lua_remove(l, -2);
    }

    // Safety: -1 为上一行 get_table 的结果槽，istable/try_replace_top_with_index
    // 只读栈顶并可在命中时以 __index/_index 表替换之（单槽改写，拓扑仍为 1 值）。
    let descended = unsafe { lua_istable!(l, -1) || try_replace_top_with_index(l) };
    if descended {
      // find(['.', ':']) 返回的 sep 必落在字符边界上
      complete_only_functions = lookup.as_bytes()[sep] == b':';
      lookup = &lookup[sep + 1..];
    } else {
      // Unable to search for keys, so stop searching
      break;
    }
  }

  // Safety: 与起始 lua_pushvalue 配平，收回栈顶表。
  unsafe { lua_pop(l, 1) };
}
