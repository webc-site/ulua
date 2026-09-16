//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:485:shrinkstackprotected`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:485-498, hand-ported)

use core::{ffi::c_void, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_rawrunprotected_ldo_alt_b::lua_d_rawrunprotected_mut, shrinkstack::shrinkstack,
  },
  type_aliases::lua_state::lua_State,
};

// C++ uses a local `struct CallContext { static void run(...) }`; a local fn is
// the Rust equivalent of that protected-call trampoline.
unsafe extern "C-unwind" fn run(l: *mut lua_State, _ud: *mut c_void) {
  unsafe {
    shrinkstack(l);
  }
}

pub(crate) unsafe fn shrinkstackprotected(l: *mut lua_State) {
  unsafe {
    // the resize call can fail on exception, in which case we will continue with original size
    let status = lua_d_rawrunprotected_mut(l, Some(run), null_mut());
    LUAU_ASSERT!(status == LuaStatus::Ok as i32 || status == LuaStatus::ErrMem as i32);
  }
}
