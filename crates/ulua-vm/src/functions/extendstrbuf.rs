//! Node: `cxx:Function:Luau.VM:VM/src/laux.cpp:449:extendstrbuf`
//!
//! Grow a `luaL_Strbuf` past its inline buffer: allocate a GC string of the next
//! size, copy the used prefix, box it on the stack at `boxloc` (inserting a slot
//! the first time it spills off the inline buffer), and repoint p/end/storage.

use core::{
  ffi::{c_char, c_int},
  ptr::copy_nonoverlapping,
};

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{
    getnextbuffersize::getnextbuffersize, lua_insert::lua_insert, lua_pushnil::lua_pushnil,
    lua_s_bufstart::lua_s_bufstart,
  },
  macros::{setsvalue::setsvalue, tsvalue::tsvalue},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn extendstrbuf(
  b: *mut LuaLStrbuf,
  additionalsize: usize,
  boxloc: c_int,
) -> *mut c_char {
  unsafe {
    let l = (*b).l;

    if !(*b).storage.is_null() {
      LUAU_ASSERT!((*b).storage.cast_const() == tsvalue!((*l).top.offset(boxloc as isize)));
    }

    let base: *mut c_char = if !(*b).storage.is_null() {
      (*(*b).storage).data.as_mut_ptr()
    } else {
      (*b).buffer.as_mut_ptr()
    };

    let capacity = (*b).end.offset_from(base) as usize;
    let nextsize = getnextbuffersize((*b).l, capacity, capacity + additionalsize);

    let new_storage = lua_s_bufstart(l, nextsize);

    let used = (*b).p.offset_from(base) as usize;
    copy_nonoverlapping(base, (*new_storage).data.as_mut_ptr(), used);

    // place the string storage at the expected position in the stack
    if base == (*b).buffer.as_mut_ptr() {
      lua_pushnil(l);
      lua_insert(l, boxloc);
    }

    setsvalue!(l, (*l).top.offset(boxloc as isize), new_storage);

    (*b).p = (*new_storage).data.as_mut_ptr().add(used);
    (*b).end = (*new_storage).data.as_mut_ptr().add(nextsize);
    (*b).storage = new_storage;

    (*b).p
  }
}
