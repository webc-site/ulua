use core::ffi::c_int;

use crate::{
  functions::{
    getnextline::getnextline, lua_a_toobject::luaA_toobject, lua_g_breakpoint::lua_g_breakpoint,
  },
  macros::{api_check::api_check, clvalue::clvalue, ttisfunction::ttisfunction},
  records::{closure::LClosure, proto::Proto},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_breakpoint(
  l: *mut lua_State,
  funcindex: c_int,
  line: c_int,
  enabled: c_int,
) -> c_int {
  unsafe {
    let func: *const TValue = luaA_toobject(l, funcindex);
    api_check!(l, ttisfunction!(func) && (*clvalue!(func)).is_c == 0);

    let cl = clvalue!(func);
    let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
    let p: *mut Proto = (*lcl).p;

    let target = getnextline(p, line);

    if target != -1 {
      lua_g_breakpoint(l, p, target, enabled != 0);
    }

    target
  }
}
