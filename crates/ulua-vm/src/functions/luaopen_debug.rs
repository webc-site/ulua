use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{db_info::db_info, db_traceback::db_traceback, lua_l_register::lua_l_register},
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

struct DblibWrapper([LuaLReg; 3]);
unsafe impl Sync for DblibWrapper {}

static DBLIB: DblibWrapper = DblibWrapper([
  LuaLReg {
    name: c"info".as_ptr(),
    func: Some(db_info),
  },
  LuaLReg {
    name: c"traceback".as_ptr(),
    func: Some(db_traceback),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_debug(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_register(l, c"debug".as_ptr(), DBLIB.0.as_ptr());
    1
  }
}
