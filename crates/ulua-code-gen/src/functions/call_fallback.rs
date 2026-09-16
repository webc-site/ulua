use core::ptr::{null, null_mut};

use ulua_common::FFlag::{LuauClosureUsageCounter, LuauNativeCodeTargetCheck};
use ulua_vm::{
  functions::lua_v_tryfunc_tm::lua_v_tryfunc_tm,
  macros::{
    clvalue::clvalue, lua_callinfo_native::LUA_CALLINFO_NATIVE,
    lua_d_checkstackfornewci::lua_d_checkstackfornewci, lua_multret::LUA_MULTRET,
    setnilvalue::setnilvalue, setobj_2_s::setobj2s, ttisfunction::ttisfunction,
  },
  records::closure::Closure,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

use crate::{functions::call_prolog::incr_ci, macros::call_fallback_yield::CALL_FALLBACK_YIELD};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_fallback(
  l: *mut lua_State,
  ra: StkId,
  mut argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  unsafe {
    if !ttisfunction!(ra as *const TValue) {
      lua_v_tryfunc_tm(l, ra);
      argtop = argtop.add(1);
    }

    let ccl = clvalue!(ra as *const TValue);

    if LuauClosureUsageCounter.get() {
      (*ccl).usage += 1;
    }

    let ci = incr_ci(l);
    (*ci).func = ra;
    (*ci).base = ra.add(1);
    (*ci).top = argtop.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;

    (*l).base = (*ci).base;
    (*l).top = argtop;

    lua_d_checkstackfornewci(l, (*ccl).stacksize as i32);
    ulua_common::LUAU_ASSERT!((*ci).top <= (*l).stack_last);

    if (*ccl).is_c == 0 {
      let p = {
        let l = &(*ccl).inner.l;
        l.p
      };

      let mut argi = (*l).top;
      let argend = (*l).base.add((*p).numparams as usize);
      while argi < argend {
        setnilvalue!(argi);
        argi = argi.add(1);
      }
      (*l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

      (*ci).savedpc = (*p).code;

      let has_native_target = if LuauNativeCodeTargetCheck.get() {
        (*p).exectarget != 0
      } else {
        !(*p).execdata.is_null()
      };

      if has_native_target {
        (*ci).flags = LUA_CALLINFO_NATIVE as u32;
      }

      ccl
    } else {
      let func = {
        let c = &(*ccl).inner.c;
        c.f
      };
      let n = match func {
        Some(f) => f(l),
        None => 0,
      };

      if n < 0 {
        return CALL_FALLBACK_YIELD as usize as *mut Closure;
      }

      let ci = (*l).ci;
      let cip = ci.sub(1);

      if LuauClosureUsageCounter.get() {
        ulua_common::LUAU_ASSERT!((*ccl).usage > 0);
        (*ccl).usage -= 1;
      }

      let mut res = (*ci).func;
      let mut vali = (*l).top.sub(n as usize);
      let valend = (*l).top;

      let mut i = nresults;
      while i != 0 && vali < valend {
        setobj2s!(l, res, vali as *const TValue);
        res = res.add(1);
        vali = vali.add(1);
        i -= 1;
      }
      while i > 0 {
        setnilvalue!(res);
        res = res.add(1);
        i -= 1;
      }

      (*l).ci = cip;
      (*l).base = (*cip).base;
      (*l).top = if nresults == LUA_MULTRET {
        res
      } else {
        (*cip).top
      };

      null_mut()
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_callFallback")]
pub unsafe extern "C-unwind" fn call_fallback_export(
  l: *mut lua_State,
  ra: StkId,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  unsafe { call_fallback(l, ra, argtop, nresults) }
}
