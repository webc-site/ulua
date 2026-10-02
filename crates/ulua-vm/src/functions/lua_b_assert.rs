use alloc::string::String;

use crate::{
  functions::lua_l_checklstring::lua_l_checklstring_ref,
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// base 库 `assert` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （实参自 1 号槽布栈、受保护帧内执行），否则抛错路径的文案槽读取失真。
/// cpp VM/src/lbaselib.cpp luaB_assert。
pub fn lua_b_assert(l: &mut LuaState) -> i32 {
  l.check_any(1);
  if !l.to_boolean(1) {
    if l.is_none_or_nil(2) {
      // SAFETY: 抛错族契约——`l` 存活且处于受保护帧（库函数调用约定），本调用不返回。
      unsafe { luaL_error!(l.as_mut_ptr(), "assertion failed!") };
    }
    let msg = lua_l_checklstring_ref(l, 2);
    let msg = String::from_utf8_lossy(msg);
    // SAFETY: 抛错族契约——`l` 存活且处于受保护帧（库函数调用约定），本调用不返回。
    unsafe { luaL_error!(l.as_mut_ptr(), "{}", msg) };
  }
  l.get_top()
}

lua_lib_fn!(pub fn lua_b_assert @ref, lua_b_assert_arm);
