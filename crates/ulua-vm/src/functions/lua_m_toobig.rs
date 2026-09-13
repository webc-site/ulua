//! Node: `cxx:Function:Luau.VM:VM/src/lmem.cpp:wrapper:luaM_toobig`
//! Source: `VM/src/lmem.cpp` (hand-ported)

use crate::{macros::lua_g_runerror::lua_g_runerror, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_toobig(l: *mut lua_State) -> ! {
  unsafe { lua_g_runerror!(l, "memory allocation error: block too big") }
}

pub use lua_m_toobig as luaM_toobig;
