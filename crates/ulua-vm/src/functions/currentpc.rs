//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:17:currentpc`
//! Source: `VM/src/ldebug.cpp`
//! Graph edges:
//! - declared_by: source_file VM/src/ldebug.cpp
//! - source_includes:
//!   - includes -> source_file VM/src/ldebug.h
//!   - includes -> source_file VM/src/lapi.h
//!   - includes -> source_file VM/src/lfunc.h
//!   - includes -> source_file VM/src/lmem.h
//!   - includes -> source_file VM/src/lgc.h
//!   - includes -> source_file VM/src/ldo.h
//!   - includes -> source_file VM/src/lbytecode.h
//!   - includes -> source_file VM/src/lstring.h
//! - incoming:
//!   - declares <- source_file VM/src/ldebug.cpp
//!   - calls <- function currentline (VM/src/ldebug.cpp)
//!   - calls <- function lua_getlocal (VM/src/ldebug.cpp)
//!   - calls <- function lua_setlocal (VM/src/ldebug.cpp)
//! - outgoing:
//!   - calls -> macro pcRel (VM/src/ldebug.h)
//!   - calls -> macro ci_func (VM/src/lstate.h)
//!   - translates_to -> rust_item currentpc

use core::ffi::c_int;

use crate::{
  macros::{ci_func::ci_func, pc_rel::pcRel},
  records::{call_info::CallInfo, closure::LClosure},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn currentpc(_l: *mut lua_State, ci: *mut CallInfo) -> c_int {
  unsafe {
    let cl = ci_func!(ci);
    let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
    pcRel!((*ci).savedpc, (*lcl).p)
  }
}
