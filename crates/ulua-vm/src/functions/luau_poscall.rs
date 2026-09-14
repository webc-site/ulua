//! Node: `cxx:Function:Luau.VM:VM/src/lvmexecute.cpp:3843:luau_poscall`
//! Source: `VM/src/lvmexecute.cpp:3843-3872` (hand-ported)

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  macros::{
    clvalue::clvalue, lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// C++ `void luau_poscall(lua_State* l, StkId first)`.
pub(crate) unsafe fn luau_poscall(l: *mut lua_State, first: StkId) {
  unsafe {
    // finish interrupted execution of `OP_CALL'
    // ci is our callinfo, cip is our parent
    let ci = (*l).ci;
    let cip = ci.sub(1);

    if FFlag::LuauClosureUsageCounter.get() {
      let cicl = clvalue!((*ci).func);
      LUAU_ASSERT!((*cicl).usage > 0);
      (*cicl).usage -= 1;
    }

    // copy return values into parent stack (but only up to nresults!), fill
    // the rest with nil
    // TODO: it might be worthwhile to handle the case when nresults==b explicitly?
    let mut res = (*ci).func;
    let mut vali = first;
    let valend = (*l).top;

    let mut i = (*ci).nresults;
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
    (*l).top = if (*ci).nresults == LUA_MULTRET {
      res
    } else {
      (*cip).top
    };
  }
}
