use crate::{
  functions::{
    db_info::db_info_arm, db_traceback::db_traceback_arm, lua_l_register::lua_l_register,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static DBLIB: [LuaLReg; 2] = [
  LuaLReg::new(b"info", db_info_arm),
  LuaLReg::new(b"traceback", db_traceback_arm),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn luaopen_debug(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"debug".as_ptr(), &DBLIB);
    1
  }
}

lua_lib_fn!(pub(crate) fn luaopen_debug, luaopen_debug_arm);
