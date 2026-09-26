use alloc::string::String;
use core::str::from_utf8;

use ulua_cli_lib::functions::{
  safe_get_table::MAX_TRAVERSAL_LIMIT, try_replace_top_with_index::try_replace_top_with_index,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_next::lua_next, lua_pushnil::lua_pushnil, lua_tolstring::lua_tolstring_ref,
    lua_type::lua_type,
  },
  macros::{lua_istable::lua_istable, lua_pop::lua_pop},
  records::lua_state::LuaState,
};

// completePartialMatches finds keys that match the specified 'prefix'
// Note: the table/object to be searched must be on the top of the Lua stack
/// # Safety
///
/// `l` 必须是有效的 `LuaState`，且待搜索的表位于栈顶。
pub(crate) unsafe fn complete_partial_matches(
  l: *mut LuaState,
  complete_only_functions: bool,
  edit_buffer: &str,
  prefix: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // cpp `for (int i = 0; i < MAX_TRAVERSAL_LIMIT; i++)`：至多跟随 MAX_TRAVERSAL_LIMIT
  // 层 __index 链，防元表环；计数无数据含义，用 range 迭代器替代手写累加。
  for _ in 0..MAX_TRAVERSAL_LIMIT {
    // lua_istable! 宏自身是安全封装；-1 为待搜索表/对象（fn /// # Safety 与
    // 各轮末的表替换），确认是表才开始遍历。
    if !lua_istable!(l, -1) {
      break;
    }

    // Safety: l 有效；起始 key=nil，push/pop 严格配平（迭代末弹 value），
    // add_completion_callback 为本帧 &mut 借用且不触栈。
    unsafe { lua_pushnil(l) };

    // Loop over all the keys in the current table
    // Safety: lua_next 以 -2 为 key 就地推进并在 -1 压入 value，返回 0 表示遍历结束。
    while unsafe { lua_next(l, -2) } != 0 {
      if unsafe { lua_type(l, -2) } == LuaType::String as i32 {
        // table, key, value
        // 键按原始字节读取：补全文本必须是合法 UTF-8，非法序列的键直接跳过
        // （避免 lossy 替换造出与实际键不符的补全项）
        // Safety: lua_type 已确认 -2 为 string，`lua_tolstring_ref` 返回其全字节
        // 切片（借用仅存活到下方 from_utf8 判定；串对象不受 GC 移动，弹 value 槽
        // 不影响 -2 键槽，与旧 (指针, 长度) 形态的存活窗口一致）。
        let key_bytes = unsafe { lua_tolstring_ref(l, -2) }.unwrap_or_default();

        let value_type = unsafe { lua_type(l, -1) };

        // 先弹回遍历槽位；后续判定与拼接全是纯字符串逻辑（安全域）
        // Safety: 与本轮 lua_next 压入的 value 配平。
        unsafe { lua_pop(l, 1) };

        let Ok(key) = from_utf8(key_bytes) else {
          continue;
        };

        // If the last separator was a ':' (i.e. a method call) then only functions should be completed.
        let required_value_type =
          !complete_only_functions || value_type == LuaType::Function as i32;

        if !key.is_empty() && required_value_type && key.starts_with(prefix) {
          // starts_with 已保证 prefix.len() 落在 key 的字符边界上
          let completed_component = &key[prefix.len()..];
          let mut completion =
            String::with_capacity(edit_buffer.len() + completed_component.len() + 1);
          completion.push_str(edit_buffer);
          completion.push_str(completed_component);
          if value_type == LuaType::Function as i32 {
            // Add an opening paren for function calls by default.
            completion.push('(');
          }
          add_completion_callback(&completion, key);
        }
      } else {
        // Safety: 非串键同样弹走本轮压入的 value，保持 -1 为下一轮 key。
        unsafe { lua_pop(l, 1) };
      }
    }

    // Replace the current table being searched with an __index table if one exists
    // Safety: -1 为遍历结束的表；命中元表 __index/_index 时单槽替换供下一轮。
    if !unsafe { try_replace_top_with_index(l) } {
      break;
    }
  }
}
