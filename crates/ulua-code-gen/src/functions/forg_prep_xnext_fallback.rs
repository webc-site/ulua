use core::ffi::c_char;

use ulua_vm::{
  macros::{clvalue::clvalue, lua_g_typeerror::luaG_typeerror, ttisfunction::ttisfunction},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

const ITERATE_OVER: *const c_char = c"iterate over".as_ptr() as *const c_char;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn forg_prep_xnext_fallback(l: *mut lua_State, ra: *mut TValue, pc: i32) {
  unsafe {
    if !ttisfunction!(ra as *const TValue) {
      let cl = clvalue!((*(*l).ci).func as *const TValue);
      let cl_l = &(*cl).inner.l;
      (*(*l).ci).savedpc = cl_l.p.as_ref().unwrap().code.add(pc as usize);

      luaG_typeerror!(l, ra as *const TValue, ITERATE_OVER);
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_forgPrepXnextFallback")]
pub unsafe extern "C-unwind" fn forg_prep_xnext_fallback_export(
  l: *mut lua_State,
  ra: *mut TValue,
  pc: i32,
) {
  unsafe { forg_prep_xnext_fallback(l, ra, pc) }
}
