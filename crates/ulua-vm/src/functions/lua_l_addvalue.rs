use core::ptr::copy_nonoverlapping;

use crate::{
  functions::{extendstrbuf::extendstrbuf, lua_tolstring::lua_tolstring},
  macros::lua_pop::lua_pop,
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_addvalue(b: *mut LuaLStrbuf) {
  unsafe {
    let l = (*b).l;

    let mut vl: usize = 0;
    let s = lua_tolstring(l, -1, &mut vl);

    if !s.is_null() {
      let current_buffer_size = (*b).end as usize - (*b).p as usize;
      if current_buffer_size < vl {
        extendstrbuf(b, vl - current_buffer_size, -2);
      }

      copy_nonoverlapping(s as *const u8, (*b).p as *mut u8, vl);
      (*b).p = (*b).p.add(vl);

      lua_pop(l, 1);
    }
  }
}
