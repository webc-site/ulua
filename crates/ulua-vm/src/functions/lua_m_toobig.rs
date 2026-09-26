//! Source: `VM/src/lmem.cpp` (hand-ported)

use crate::{macros::lua_g_runerror::lua_g_runerror, records::lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_toobig(l: *mut LuaState) -> ! {
  unsafe { lua_g_runerror!(l, "memory allocation error: block too big") }
}
