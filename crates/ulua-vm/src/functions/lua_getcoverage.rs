use core::ffi::{c_int, c_void};

use crate::{
  functions::{getcoverage::getcoverage, getmaxline::getmaxline, lua_a_toobject::luaA_toobject},
  macros::{
    api_check::api_check, clvalue::clvalue, lua_m_freearray::luaM_freearray,
    lua_m_newarray::luaM_newarray, ttisfunction::ttisfunction,
  },
  records::closure::LClosure,
  type_aliases::{lua_coverage::LuaCoverage, lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getcoverage(
  l: *mut lua_State,
  funcindex: c_int,
  context: *mut c_void,
  callback: LuaCoverage,
) {
  unsafe {
    let func: *const TValue = luaA_toobject(l, funcindex);
    api_check!(l, ttisfunction!(func) && (*clvalue!(func)).is_c == 0);

    let cl = clvalue!(func);
    let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
    let p = (*lcl).p;

    let size = getmaxline(p) as usize + 1;
    if size == 0 {
      return;
    }

    let buffer = luaM_newarray!(l, size, c_int, 0);

    getcoverage(p, 0, buffer, size, context, callback);

    luaM_freearray!(l, buffer as *mut c_void, size, c_int, 0);
  }
}
