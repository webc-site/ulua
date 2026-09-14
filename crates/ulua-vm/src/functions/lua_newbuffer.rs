//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:1477:lua_newbuffer`
//!
//! `lua_newbuffer` — allocate a managed buffer object of `sz` bytes, push it on
//! the stack, and return a pointer to its data. Runs a GC step and the thread
//! write-barrier first, exactly like the C++ public API.

use core::ffi::c_void;

use crate::{
  functions::{lua_b_newbuffer::lua_b_newbuffer, lua_concat::lua_c_threadbarrier_lapi},
  macros::{api_incr_top::api_incr_top, lua_c_check_gc::luaC_checkGC, setbufvalue::setbufvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_newbuffer(l: *mut lua_State, sz: usize) -> *mut c_void {
  unsafe {
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    let b = lua_b_newbuffer(l, sz);
    setbufvalue!(l, (*l).top, b);
    api_incr_top!(l);
    (*b).data.as_mut_ptr() as *mut c_void
  }
}
