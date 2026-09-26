use crate::{
  functions::{db_info::db_info, db_traceback::db_traceback, lua_l_register::lua_l_register},
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static DBLIB: [LuaLReg; 2] = [
  LuaLReg::new(b"info", db_info),
  LuaLReg::new(b"traceback", db_traceback),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_debug(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"debug".as_ptr(), &DBLIB);
    1
  }
}
