use crate::{
  functions::{
    lua_l_register::lua_l_register, os_clock::os_clock_arm, os_date::os_date_arm,
    os_difftime::os_difftime_arm, os_time::os_time_arm,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn luaopen_os(l: *mut LuaState) -> i32 {
  unsafe {
    // Faithful port of syslib[] in loslib.cpp.
    let syslib: [LuaLReg; 4] = [
      LuaLReg::new(b"clock", os_clock_arm),
      LuaLReg::new(b"date", os_date_arm),
      LuaLReg::new(b"difftime", os_difftime_arm),
      LuaLReg::new(b"time", os_time_arm),
    ];

    lua_l_register(l, c"os".as_ptr(), &syslib);
    1
  }
}

lua_lib_fn!(pub(crate) fn luaopen_os, luaopen_os_arm);
