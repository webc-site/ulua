use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice, c_slice_mut, index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi,
    lua_f_new_lclosure::luaF_newLclosure,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::luaC_checkGC,
    setclvalue::setclvalue, setobj_2_n::setobj2n,
  },
  records::{closure::Closure, lua_t_value::TValue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_clonefunction(l: *mut lua_State, idx: c_int) {
  unsafe {
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    let p: StkId = index2addr(l, idx);
    let cl = core::ptr::addr_of_mut!((*(*p).value.gc).cl) as *mut Closure;
    api_check!(
      l,
      (*p).tt() == LuaType::Function as c_int && (*cl).is_c == 0
    );
    let lc = core::ptr::addr_of!((*cl).inner.l);
    let newcl: *mut Closure = luaF_newLclosure(l, (*cl).nupvalues as c_int, (*l).gt, (*lc).p);
    let newlc = core::ptr::addr_of_mut!((*newcl).inner.l);
    // SAFETY：newLclosure 按 nupvalues 分配 uprefs，与源 closure 等长。
    for (dst, src) in c_slice_mut((*newlc).uprefs.as_mut_ptr(), (*cl).nupvalues as usize)
      .iter_mut()
      .zip(c_slice((*lc).uprefs.as_ptr(), (*cl).nupvalues as usize))
    {
      setobj2n!(l, dst as *mut TValue, src as *const TValue as *mut TValue);
    }
    setclvalue!(l, (*l).top, newcl);
    api_incr_top!(l);
  }
}
