//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:313:luaG_readonlyerror`
//! Source: `VM/src/ldebug.cpp:313-316` (hand-ported)

use crate::{macros::lua_g_runerror::lua_g_runerror, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_readonlyerror(l: *mut lua_State) -> ! {
  unsafe { lua_g_runerror!(l, "attempt to modify a readonly table") }
}

pub use lua_g_readonlyerror as luaG_readonlyerror;
