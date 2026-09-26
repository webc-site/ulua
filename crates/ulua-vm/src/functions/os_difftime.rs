use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber, lua_l_optnumber::lua_l_optnumber,
    lua_pushnumber::lua_pushnumber,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn os_difftime(l: *mut LuaState) -> i32 {
  unsafe {
    let t1 = lua_l_checknumber(l, 1);
    let t2 = lua_l_optnumber(l, 2, 0.0);

    // difftime in C returns the difference in seconds (t1 - t2) as a double.
    // Since we are targeting wasm32-unknown-unknown and portable environments,
    // and the input numbers are already doubles from the Lua stack, we can
    // perform the subtraction directly.
    // DELIBERATE DEVIATION：省去 cpp loslib.cpp:209 的 (time_t) 往返——输入
    // 已是 double，|t| < 2^53 逐位一致；超域为 C UB 角落，直取 double 相减。
    let result = t1 - t2;

    lua_pushnumber(l, result);
    1
  }
}

lua_lib_fn!(pub fn os_difftime, os_difftime_arm);
