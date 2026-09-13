use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    lua_l_register::lua_l_register, os_clock::os_clock, os_date::os_date, os_difftime::os_difftime,
    os_time::os_time,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn luaopen_os(l: *mut lua_State) -> c_int {
  unsafe {
    // Faithful port of syslib[] in loslib.cpp.
    let syslib: [LuaLReg; 5] = [
      LuaLReg {
        name: c"clock".as_ptr(),
        func: Some(os_clock),
      },
      LuaLReg {
        name: c"date".as_ptr(),
        func: Some(os_date),
      },
      LuaLReg {
        name: c"difftime".as_ptr(),
        func: Some(os_difftime),
      },
      LuaLReg {
        name: c"time".as_ptr(),
        func: Some(os_time),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    lua_l_register(l, c"os".as_ptr(), syslib.as_ptr());
    1
  }
}
