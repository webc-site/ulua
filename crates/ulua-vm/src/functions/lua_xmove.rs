use core::ffi::c_int;

use crate::{
  functions::{
    c_slice, c_slice_mut, ensure_stack::ensure_stack_impl, lua_concat::lua_c_threadbarrier_lapi,
  },
  macros::{api_check::api_check, api_checknelems::api_checknelems, setobj_2_s::setobj_2_s},
  records::lua_t_value::TValue,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_xmove"))]
pub unsafe fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int) {
  unsafe {
    api_check!(from, n >= 0);

    if from == to {
      return;
    }

    api_checknelems!(from, n);
    api_check!(from, (*from).global == (*to).global);

    lua_c_threadbarrier_lapi(to);

    // cpp `ensure_stack_impl(to, from, n)`：目标帧不够时扩容，失败在 from 上抛错
    ensure_stack_impl(to, from, n);

    let ttop = (*to).top;
    let ftop = (*from).top.offset(-(n as isize));

    // SAFETY：from 栈自 ftop 起 n 个值有效；to 栈保证容纳 n 个新值（checkstack 已过）。
    for (dst, src) in c_slice_mut(ttop, n as usize)
      .iter_mut()
      .zip(c_slice(ftop, n as usize))
    {
      setobj_2_s!(to, dst as *mut TValue, src as *const TValue as *mut TValue);
    }

    (*from).top = ftop;
    (*to).top = ttop.offset(n as isize);
  }
}
