use core::ffi::c_int;

use crate::{
  functions::{
    auxresume::auxresume, coresumefinish::coresumefinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::{
    cast_int::cast_int, co_status_break::CO_STATUS_BREAK, lua_l_argexpected::luaL_argexpected,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn coresumey(l: *mut lua_State) -> c_int {
  unsafe {
    let co = lua_tothread(l, 1);
    luaL_argexpected!(l, !co.is_null(), 1, "thread");
    let narg = cast_int!((*l).top.offset_from((*l).base)) - 1;
    let r = auxresume(l, co, narg);

    if r == CO_STATUS_BREAK {
      return interrupt_thread(l, co);
    }

    coresumefinish(l, r)
  }
}
