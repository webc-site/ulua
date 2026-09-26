use core::ptr::{addr_of, addr_of_mut};

use crate::{
  functions::{
    c_slice, c_slice_mut, ensure_stack::ensure_stack, index_2_addr::index_2_addr,
    lapi_barrier::lua_c_threadbarrier_lapi, lua_f_new_lclosure::lua_f_new_lclosure,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc,
    setclvalue::setclvalue, setobj_2_n::setobj2n,
  },
  records::{closure::Closure, lua_state::LuaState, lua_t_value::TValue},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`（会 `lua_c_check_gc`、线程屏障、`ensure_stack(1)`、`lua_f_new_lclosure` 分配）；
/// `idx` 经 `index_2_addr` 解析出的槽须为 Lua（非 C）闭包 TValue——即 `(*p).value.gc` 指向存活 `Closure`
/// 且 `is_c==0`，其 `inner.l.uprefs[0..nupvalues]` 均为存活对象（逐一 `setobj2n` 复制到新闭包）。
/// cpp `lapi.cpp:2115`。
pub unsafe fn lua_clonefunction(l: *mut LuaState, idx: i32) {
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    let p: StkId = index_2_addr(l, idx);
    let Some(cl) = (*(*p).value.gc).as_closure_mut() else {
      api_check!(l, false);
      return;
    };
    api_check!(l, cl.is_c == 0);
    let lc = addr_of!(cl.inner.l);
    let newcl: *mut Closure = lua_f_new_lclosure(l, cl.nupvalues as i32, (*l).gt, (*lc).p);
    let newlc = addr_of_mut!((*newcl).inner.l);
    // Safety:newLclosure 按 nupvalues 分配 uprefs，与源 closure 等长。
    for (dst, src) in c_slice_mut((*newlc).uprefs.as_mut_ptr(), cl.nupvalues as usize)
      .iter_mut()
      .zip(c_slice((*lc).uprefs.as_ptr(), cl.nupvalues as usize))
    {
      setobj2n!(l, dst as *mut TValue, src as *const TValue as *mut TValue);
    }
    setclvalue!(l, (*l).top, newcl);
    api_incr_top!(l);
  }
}
