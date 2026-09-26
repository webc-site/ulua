use crate::{
  functions::{
    lua_l_register::lua_l_register, os_clock::os_clock, os_date::os_date, os_difftime::os_difftime,
    os_time::os_time,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_os(l: *mut LuaState) -> i32 {
  unsafe {
    // Faithful port of syslib[] in loslib.cpp.
    let syslib: [LuaLReg; 4] = [
      LuaLReg::new(b"clock", os_clock),
      LuaLReg::new(b"date", os_date),
      LuaLReg::new(b"difftime", os_difftime),
      LuaLReg::new(b"time", os_time),
    ];

    lua_l_register(l, c"os".as_ptr(), &syslib);
    1
  }
}
