use crate::{
  functions::{lua_clock::lua_clock, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn os_clock(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, lua_clock());
    1
  }
}
