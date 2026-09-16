use core::mem::size_of;

use crate::{
  functions::{lua_m_realloc::lua_m_realloc_, runerror::runerror},
  macros::setnilvalue::setnilvalue,
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

const MAXSIZE: i32 = 1 << 26;

pub(crate) unsafe fn setarrayvector(l: *mut lua_State, t: *mut LuaTable, size: i32) {
  unsafe {
    if size > MAXSIZE {
      runerror(l, c"table overflow".as_ptr());
    }

    let oldsize = (*t).sizearray;
    let newarray = lua_m_realloc_(
      l,
      (*t).array as *mut u8,
      oldsize as usize * size_of::<TValue>(),
      size as usize * size_of::<TValue>(),
      (*t).memcat,
    ) as *mut TValue;

    (*t).array = newarray;

    for i in oldsize..size {
      setnilvalue!((*t).array.add(i as usize));
    }

    (*t).sizearray = size;
  }
}
