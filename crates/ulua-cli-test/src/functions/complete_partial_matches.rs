use alloc::string::ToString;
use core::{slice::from_raw_parts, str::from_utf8};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_next::lua_next, lua_pushnil::lua_pushnil, lua_tolstring::lua_tolstring, lua_type::lua_type,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::lua_State,
};

use crate::functions::try_replace_top_with_index::try_replace_top_with_index;
pub fn complete_partial_matches(
  l: *mut lua_State,
  complete_only_functions: bool,
  edit_buffer: &str,
  prefix: &str,
  add_completion_callback: &mut dyn FnMut(&str, &str),
) {
  unsafe {
    for _ in 0..50 {
      if lua_type(l, -1) != LuaType::Table as i32 {
        break;
      }

      lua_pushnil(l);

      while lua_next(l, -2) != 0 {
        if lua_type(l, -2) == LuaType::String as i32 {
          let mut len: usize = 0;
          let key_ptr = lua_tolstring(l, -2, &mut len);
          let key_slice = if key_ptr.is_null() {
            &[]
          } else {
            from_raw_parts(key_ptr as *const u8, len)
          };
          let Ok(key) = from_utf8(key_slice) else {
            lua_pop(l, 1);
            continue;
          };
          let value_type = lua_type(l, -1);
          let required_value_type =
            !complete_only_functions || value_type == LuaType::Function as i32;

          if !key.is_empty() && required_value_type && key.starts_with(prefix) {
            let completed_component = &key[prefix.len()..];
            let mut completion = edit_buffer.to_string();
            completion.push_str(completed_component);

            if value_type == LuaType::Function as i32 {
              completion.push('(');
            }

            add_completion_callback(&completion, key);
          }
        }

        lua_pop(l, 1);
      }

      if !try_replace_top_with_index(l) {
        break;
      }
    }
  }
}
