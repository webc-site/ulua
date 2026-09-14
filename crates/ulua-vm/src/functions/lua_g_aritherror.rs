//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:264:luaG_aritherror`
//! Source: `VM/src/ldebug.cpp:264-275` (hand-ported; `luaT_eventname[op]`
//! is read from `g->tmname[op]`, built from the same string table)

use core::ffi::{CStr, c_char};

use crate::{
  enums::tms::TMS,
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_aritherror(
  l: *mut lua_State,
  p1: *const TValue,
  p2: *const TValue,
  op: TMS,
) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);
    // skip __ from metamethod name
    let opname = getstr((*(*l).global).tmname[op as usize]).add(2);

    if t1 == t2 {
      // C++ compares interned typename pointers
      lua_g_runerror!(
        l,
        "attempt to perform arithmetic ({}) on {}",
        CStr::from_ptr(opname).to_string_lossy(),
        CStr::from_ptr(t1).to_string_lossy()
      )
    } else {
      lua_g_runerror!(
        l,
        "attempt to perform arithmetic ({}) on {} and {}",
        CStr::from_ptr(opname).to_string_lossy(),
        CStr::from_ptr(t1).to_string_lossy(),
        CStr::from_ptr(t2).to_string_lossy()
      )
    }
  }
}

pub use lua_g_aritherror as luaG_aritherror;
