//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:277:luaG_ordererror`
//! Source: `VM/src/ldebug.cpp:277-284` (hand-ported)

use core::ffi::{CStr, c_char};

use crate::{
  enums::tms::TMS,
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::lua_g_runerror::lua_g_runerror,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_ordererror(
  l: *mut lua_State,
  p1: *const TValue,
  p2: *const TValue,
  op: TMS,
) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);
    let opname: &str = if op == TMS::TmLt {
      "<"
    } else if op == TMS::TmLe {
      "<="
    } else {
      "=="
    };

    lua_g_runerror!(
      l,
      "attempt to compare {} {} {}",
      CStr::from_ptr(t1).to_string_lossy(),
      opname,
      CStr::from_ptr(t2).to_string_lossy()
    )
  }
}

pub use lua_g_ordererror as luaG_ordererror;
