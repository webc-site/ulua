use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_l_getmetafield::lua_l_getmetafield, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
    lua_rawget::lua_rawget, lua_remove::lua_remove, lua_replace::lua_replace,
  },
  macros::{lua_isnil::lua_isnil, lua_istable::lua_istable, lua_pop::lua_pop},
  records::lua_state::lua_State,
};

const MAX_TRAVERSAL_LIMIT: c_int = 50;

pub fn safe_get_table(l: *mut lua_State, table_index: c_int) {
  unsafe {
    lua_pushvalue(l, table_index); // Duplicate the table

    // The loop invariant is that the table to search is at -1
    // and the key is at -2.
    let mut loop_count: c_int = 0;
    loop {
      lua_pushvalue(l, -2); // Duplicate the key
      lua_rawget(l, -2); // Try to find the key

      if !lua_isnil!(l, -1) || loop_count >= MAX_TRAVERSAL_LIMIT {
        break;
      }

      lua_pop(l, 1); // Pop the nil result
      if lua_l_getmetafield(l, -1, c"__index".as_ptr()) == 0 {
        lua_pushnil(l);
        break;
      } else if lua_istable!(l, -1) {
        // Replace the current table being searched with __index table
        lua_replace(l, -2);
      } else {
        lua_pop(l, 1); // Pop the value
        lua_pushnil(l);
        break;
      }

      loop_count += 1;
    }

    lua_remove(l, -2); // Remove the table
    lua_remove(l, -2); // Remove the original key
  }
}
