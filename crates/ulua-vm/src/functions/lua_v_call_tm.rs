use core::ptr::null_mut;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::lua_d_check_cstack::luaD_checkCstack,
  macros::{
    clvalue::clvalue, incr_ci::incr_ci, lua_d_checkstack::luaD_checkstack,
    lua_minstack::LUA_MINSTACK, luai_maxccalls::LUAI_MAXCCALLS, setnilvalue::setnilvalue,
    setobj_2_s::setobj2s, ttisfunction::ttisfunction,
  },
  records::{
    closure::{CClosure, Closure},
    lua_state::lua_State,
  },
  type_aliases::t_value::TValue,
};

/// C++ `LUAU_NOINLINE void luaV_callTM(lua_State* l, int nparams, int res)`.
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_call_tm(l: *mut lua_State, nparams: i32, res: i32) {
  unsafe {
    // LUAU_NOINLINE is handled by the attribute on the function
    (*l).n_ccalls += 1;

    if (*l).n_ccalls >= LUAI_MAXCCALLS as u16 {
      luaD_checkCstack(l);
    }

    luaD_checkstack!(l, LUA_MINSTACK);

    let top = (*l).top;
    let fun = top.sub(nparams as usize).sub(1);

    let ci = incr_ci!(l);
    (*ci).func = fun;
    (*ci).base = fun.add(1);
    (*ci).top = top.add(LUA_MINSTACK as usize);
    (*ci).savedpc = null_mut();
    (*ci).flags = 0;
    (*ci).nresults = if res >= 0 { 1 } else { 0 };
    LUAU_ASSERT!((*ci).top <= (*l).stack_last);

    let mut ccl: *mut Closure = null_mut();
    if FFlag::LuauClosureUsageCounter.get() {
      ccl = clvalue!(fun);
      (*ccl).usage += 1;
    }

    LUAU_ASSERT!(ttisfunction!((*ci).func));
    LUAU_ASSERT!((*clvalue!((*ci).func)).is_c != 0);

    (*l).base = fun.add(1);
    LUAU_ASSERT!((*l).top == (*l).base.add(nparams as usize));

    let cl = clvalue!(fun);
    let c = core::ptr::addr_of!((*cl).inner.c).cast::<CClosure>();
    let func = (*c).f;
    let n = func.unwrap()(l);
    LUAU_ASSERT!(n >= 0); // yields should have been blocked by n_ccalls

    // ci is our callinfo, cip is our parent
    // note that we read l->ci again since it may have been reallocated by the call
    let cip = (*l).ci.sub(1);

    if FFlag::LuauClosureUsageCounter.get() {
      LUAU_ASSERT!((*ccl).usage > 0);
      (*ccl).usage -= 1;
    }

    // copy return value into parent stack
    if res >= 0 {
      if n > 0 {
        setobj2s!(
          l,
          (*cip).base.add(res as usize),
          (*l).top.sub(n as usize) as *const TValue
        );
      } else {
        setnilvalue!((*cip).base.add(res as usize));
      }
    }

    (*l).ci = cip;
    (*l).base = (*cip).base;
    (*l).top = (*cip).top;

    (*l).n_ccalls -= 1;
  }
}
