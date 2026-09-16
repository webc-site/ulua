use core::ptr::null;

use ulua_common::FFlag::LuauClosureUsageCounter;
use ulua_vm::{
  functions::{lua_d_grow_ci::lua_d_grow_ci, lua_v_tryfunc_tm::lua_v_tryfunc_tm},
  macros::{
    clvalue::clvalue, lua_d_checkstackfornewci::lua_d_checkstackfornewci,
    ttisfunction::ttisfunction,
  },
  records::{call_info::CallInfo, closure::Closure},
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_prolog(
  l: *mut lua_State,
  ra: *mut TValue,
  mut argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  unsafe {
    if !ttisfunction!(ra as *const TValue) {
      lua_v_tryfunc_tm(l, ra);
      argtop = argtop.add(1);
    }

    let ccl = clvalue!(ra as *const TValue);

    let ci = incr_ci(l);
    (*ci).func = ra;
    (*ci).base = ra.add(1);
    (*ci).top = argtop.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;

    if LuauClosureUsageCounter.get() {
      (*ccl).usage += 1;
    }

    (*l).base = (*ci).base;
    (*l).top = argtop;

    lua_d_checkstackfornewci(l, (*ccl).stacksize as i32);

    ccl
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
///
/// C++ `++L->ci`：无 condhardstacktests 分支（与 VM 的 `incr_ci!` 宏语义不同），
/// 供 call_prolog / call_fallback 复用。
pub(crate) unsafe fn incr_ci(l: *mut lua_State) -> *mut CallInfo {
  unsafe {
    if (*l).ci == (*l).end_ci {
      lua_d_grow_ci(l);
    } else {
      (*l).ci = (*l).ci.add(1);
    }

    (*l).ci
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_callProlog")]
pub unsafe extern "C-unwind" fn call_prolog_export(
  l: *mut lua_State,
  ra: *mut TValue,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  unsafe { call_prolog(l, ra, argtop, nresults) }
}
