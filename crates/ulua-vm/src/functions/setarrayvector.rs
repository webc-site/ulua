use core::{
  ffi::{c_char, c_int},
  mem::size_of,
};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_throw_ldo::luaD_throw, lua_g_pusherror::lua_g_pusherror, lua_m_realloc::lua_m_realloc_,
  },
  macros::setnilvalue::setnilvalue,
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

const MAXSIZE: i32 = 1 << 26;

unsafe fn runerror(l: *mut lua_State, msg: *const c_char) -> ! {
  unsafe {
    lua_g_pusherror(l, msg);
    luaD_throw(l, LuaStatus::ErrRun as c_int);
  }
}

pub(crate) unsafe fn setarrayvector(l: *mut lua_State, t: *mut LuaTable, size: c_int) {
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
