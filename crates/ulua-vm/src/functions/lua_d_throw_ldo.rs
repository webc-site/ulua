//! Node: `cxx:Function:Luau.VM:VM/src/ldo.cpp:162:lua_d_throw`
//! Source: `VM/src/ldo.cpp` (ldo.cpp:162-165, hand-ported; C++-exceptions build flavor,
//! matching the catch_unwind-based luaD_rawrunprotected)

use core::ffi::c_int;
use std::panic::panic_any;

use crate::{records::lua_exception::lua_exception, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_throw(l: *mut lua_State, errcode: c_int) -> ! {
  panic_any(lua_exception::new(l, errcode));
}

pub use lua_d_throw as luaD_throw;
