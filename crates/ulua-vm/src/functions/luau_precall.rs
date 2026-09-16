//! Node: `cxx:Function:Luau.VM:VM/src/lvmexecute.cpp:3757:luau_precall`
//! Source: `VM/src/lvmexecute.cpp:3757-3841` (hand-ported)

use core::{ffi::c_int, ptr::null};

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::lua_v_tryfunc_tm::lua_v_tryfunc_tm,
  macros::{
    clvalue::clvalue, incr_ci::incr_ci, lua_callinfo_native::LUA_CALLINFO_NATIVE,
    lua_d_checkstackfornewci::luaD_checkstackfornewci, pcrc::PCRC, pcrlua::PCRLUA,
    pcryield::PCRYIELD, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s,
    ttisfunction::ttisfunction,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// C++ `int luau_precall(lua_State* l, StkId func, int nresults)`.
pub(crate) unsafe fn luau_precall(l: *mut lua_State, func: StkId, nresults: c_int) -> c_int {
  unsafe {
    if !ttisfunction!(func as *const TValue) {
      lua_v_tryfunc_tm(l, func);
      // l->top is incremented by tryfuncTM
    }

    let ccl = clvalue!(func as *const TValue);

    incr_ci!(l);
    let ci = (*l).ci;
    (*ci).func = func;
    (*ci).base = func.add(1);
    (*ci).top = (*l).top.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;
    if FFlag::LuauClosureUsageCounter.get() {
      (*ccl).usage += 1;
    }

    (*l).base = (*ci).base;
    // Note: l->top is assigned externally

    luaD_checkstackfornewci(l, (*ccl).stacksize as i32);
    LUAU_ASSERT!((*ci).top <= (*l).stack_last);

    if (*ccl).is_c == 0 {
      let p = {
        let l = &(*ccl).inner.l;
        l.p
      };

      // fill unused parameters with nil
      let mut argi = (*l).top;
      let argend = (*l).base.add((*p).numparams as usize);
      while argi < argend {
        setnilvalue!(argi); // complete missing arguments
        argi = argi.add(1);
      }
      (*l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

      (*ci).savedpc = (*p).code;

      // VM_HAS_NATIVE
      if (*p).exectarget != 0 && !(*p).execdata.is_null() {
        (*ci).flags = LUA_CALLINFO_NATIVE as u32;
      }

      PCRLUA
    } else {
      let f = {
        let c = &(*ccl).inner.c;
        c.f
      };
      let n = match f {
        Some(f) => f(l),
        None => 0,
      };

      // yield
      if n < 0 {
        return PCRYIELD;
      }

      // ci is our callinfo, cip is our parent
      let ci = (*l).ci;
      let cip = ci.sub(1);

      if FFlag::LuauClosureUsageCounter.get() {
        LUAU_ASSERT!((*ccl).usage > 0);
        (*ccl).usage -= 1;
      }

      // copy return values into parent stack (but only up to nresults!),
      // fill the rest with nil
      // TODO: it might be worthwhile to handle the case when nresults==b explicitly?
      let mut res = (*ci).func;
      let mut vali = (*l).top.sub(n as usize);
      let valend = (*l).top;

      let mut i = nresults;
      while i != 0 && vali < valend {
        setobj_2_s!(l, res, vali as *const TValue);
        res = res.add(1);
        vali = vali.add(1);
        i -= 1;
      }
      while i > 0 {
        setnilvalue!(res);
        res = res.add(1);
        i -= 1;
      }

      // pop the stack frame
      (*l).ci = cip;
      (*l).base = (*cip).base;
      (*l).top = res;

      PCRC
    }
  }
}
