use core::ffi::c_char;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue,
    lua_remove::lua_remove, lua_type::lua_type,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_minstack::LUA_MINSTACK, lua_pop::lua_pop},
  records::lua_state::lua_State,
};

use crate::functions::{
  complete_partial_matches::complete_partial_matches, safe_get_table::safe_get_table,
  try_replace_top_with_index::try_replace_top_with_index,
};
pub fn complete_indexer(
  l: *mut lua_State,
  edit_buffer: &str,
  add_completion_callback: &mut dyn FnMut(&str, &str),
) {
  let mut lookup: &str = edit_buffer;
  let mut complete_only_functions = false;

  unsafe {
    lua_checkstack(l, LUA_MINSTACK);

    // Push the global variable table to begin the search
    lua_pushvalue(l, LUA_GLOBALSINDEX);

    loop {
      let sep_idx = match lookup.find(['.', ':']) {
        Some(i) => i,
        None => {
          complete_partial_matches(
            l,
            complete_only_functions,
            edit_buffer,
            lookup,
            add_completion_callback,
          );
          break;
        }
      };

      let prefix = &lookup[..sep_idx];

      // find the key in the table
      lua_pushlstring(l, prefix.as_ptr() as *const c_char, prefix.len());
      safe_get_table(l, -2);
      lua_remove(l, -2);

      let is_table = lua_type(l, -1) == (LuaType::Table as i32);

      if is_table || try_replace_top_with_index(l) {
        complete_only_functions = lookup.as_bytes()[sep_idx] == b':';
        lookup = &lookup[sep_idx + 1..];
      } else {
        // Unable to search for keys, so stop searching
        break;
      }
    }

    lua_pop(l, 1);
  }
}
