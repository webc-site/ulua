//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:256:luaG_concaterror`
//! Source: `VM/src/ldebug.cpp:256-262` (hand-ported)

use core::ffi::{CStr, c_char};

use crate::{
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::lua_g_runerror::lua_g_runerror,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_concaterror(l: *mut lua_State, p1: StkId, p2: StkId) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);

    lua_g_runerror!(
      l,
      "attempt to concatenate {} with {}",
      CStr::from_ptr(t1).to_string_lossy(),
      CStr::from_ptr(t2).to_string_lossy()
    )
  }
}

pub use lua_g_concaterror as luaG_concaterror;
