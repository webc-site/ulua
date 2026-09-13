//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:292:lua_close`
//! Source: `VM/src/lstate.cpp:292-297` (hand-ported)

use crate::{
  functions::{close_state::close_state, lua_f_close::lua_f_close},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_close(l: *mut lua_State) {
  unsafe {
    let l = (*(*l).global).mainthread; // only the main thread can be closed
    lua_f_close(l, (*l).stack); // close all upvalues for this thread
    close_state(l);
  }
}
