use alloc::string::String;
use core::{ffi::c_int, slice::from_raw_parts, str::from_utf8};

use ulua_cli_lib::functions::{
  safe_get_table::MAX_TRAVERSAL_LIMIT, try_replace_top_with_index::try_replace_top_with_index,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_next::lua_next, lua_pushnil::lua_pushnil, lua_tolstring::lua_tolstring, lua_type::lua_type,
  },
  macros::{lua_istable::lua_istable, lua_pop::lua_pop},
  type_aliases::lua_state::lua_State,
};

// completePartialMatches finds keys that match the specified 'prefix'
// Note: the table/object to be searched must be on the top of the Lua stack
/// # Safety
///
/// `l` 必须是有效的 `lua_State`，且待搜索的表位于栈顶。
pub(crate) unsafe fn complete_partial_matches(
  l: *mut lua_State,
  complete_only_functions: bool,
  edit_buffer: &str,
  prefix: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  unsafe {
    let mut i: c_int = 0;
    while i < MAX_TRAVERSAL_LIMIT && lua_istable!(l, -1) {
      // table, key
      lua_pushnil(l);

      // Loop over all the keys in the current table
      while lua_next(l, -2) != 0 {
        if lua_type(l, -2) == LuaType::String as i32 {
          // table, key, value
          // 键按原始字节读取：补全文本必须是合法 UTF-8，非法序列的键直接跳过
          // （避免 lossy 替换造出与实际键不符的补全项）
          let mut len: usize = 0;
          let key_ptr = lua_tolstring(l, -2, &mut len);
          // SAFETY: lua_type 已确认为 string，返回缓冲为 len 字节
          let key_bytes = if key_ptr.is_null() {
            &[]
          } else {
            from_raw_parts(key_ptr as *const u8, len)
          };
          let Ok(key) = from_utf8(key_bytes) else {
            lua_pop(l, 1);
            continue;
          };

          let value_type = lua_type(l, -1);

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
        }
        lua_pop(l, 1);
      }

      // Replace the current table being searched with an __index table if one exists
      if !try_replace_top_with_index(l) {
        break;
      }

      i += 1;
    }
  }
}
