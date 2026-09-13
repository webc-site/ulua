use core::ffi::c_char;

use crate::{
  functions::{lua_gettop::lua_gettop, lua_l_tolstring::lua_l_tolstring, writestring::writestring},
  macros::lua_pop::lua_pop,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_print")]
pub(crate) unsafe extern "C-unwind" fn lua_b_print(l: *mut lua_State) -> i32 {
  unsafe {
    let n = lua_gettop(l);
    for i in 1..=n {
      let mut len = 0;
      let s = lua_l_tolstring(l, i, &mut len);
      if i > 1 {
        writestring("\t".as_ptr() as *const c_char, 1);
      }
      writestring(s, len);
      lua_pop(l, 1);
    }
    writestring("\n".as_ptr() as *const c_char, 1);
    0
  }
}
