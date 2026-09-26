use core::ptr::{addr_of, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::value_view::ValueView,
  functions::lua_d_check_cstack::lua_d_check_cstack,
  macros::{
    incr_ci::incr_ci, lua_d_checkstack::luaD_checkstack, lua_minstack::LUA_MINSTACK,
    luai_maxccalls::LUAI_MAXCCALLS, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s,
  },
  records::{
    closure::{CClosure, Closure},
    lua_state::LuaState,
  },
  type_aliases::t_value::TValue,
};

/// C++ `LUAU_NOINLINE void luaV_callTM(LuaState* l, int nparams, int res)`.
/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
#[inline(never)] // 对应 cpp LUAU_NOINLINE：控制栈帧体积，勿内联
pub unsafe fn lua_v_call_tm(l: *mut LuaState, nparams: i32, res: i32) {
  // Safety: 契约保证 `l` 为存活调用帧、tm/左值/右值按约定可读可写，块内重入调用不越本帧栈界
  unsafe {
    (*l).n_ccalls += 1;

    if (*l).n_ccalls >= LUAI_MAXCCALLS as u16 {
      lua_d_check_cstack(l);
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

    // cpp `lua_assert(ttisfunction(ci->func))` + `lua_assert(clvalue(ci->func)->isc)`：
    // §11 pass B 把这段 tag→payload 读链收敛为一次 [`ValueView`] 读取——Function 变体本身
    // 即 `ttisfunction` 判据，`is_c` 断言在臂内。随后只透传 `*const Closure` 裸指针：被调
    // C 闭包可写闭包体、可触发 GC 甚至搬栈，视图借用不外溢（同 value_view 寿命契约）。
    let cl = match ValueView::from_tvalue(&*fun) {
      ValueView::Function(cl) => {
        LUAU_ASSERT!(cl.is_c != 0);
        cl as *const Closure
      }
      // 非函数 tag：cpp 由上述 lua_assert 兜底，release 下仍按 `clvalue!` 读 union 成员；
      // 断言原样保留，故该臂与旧链在 debug 下行为一致
      _ => {
        LUAU_ASSERT!((*fun).is_function());
        (*fun).as_closure_ptr()
      }
    };

    (*l).base = fun.add(1);
    LUAU_ASSERT!((*l).top == (*l).base.add(nparams as usize));

    let c = addr_of!((*cl).inner.c).cast::<CClosure>();
    let func = (*c).f;
    // f 非空由 CClosure 构造契约保证（lua_pushcclosure 必设 .f，cpp luaD_callnoyield 同款直接调用）
    let n = func.expect("CClosure.f 由 lua_pushcclosure 构造契约保证非空")(l);
    LUAU_ASSERT!(n >= 0); // yields should have been blocked by n_ccalls

    // ci is our callinfo, cip is our parent
    // note that we read l->ci again since it may have been reallocated by the call
    let cip = (*l).ci.sub(1);

    // copy return value into parent stack
    if res >= 0 {
      if n > 0 {
        setobj_2_s!(
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
