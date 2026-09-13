use core::ffi::c_int;

use crate::{
  functions::{c_slice, c_slice_mut, lua_c_barrierback::lua_c_barrierback},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, blackbit::BLACKBIT,
    setobj_2_s::setobj_2_s,
  },
  records::lua_t_value::TValue,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_xmove")]
pub unsafe fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int) {
  unsafe {
    api_check!(from, n >= 0);

    if from == to {
      return;
    }

    api_checknelems!(from, n);
    api_check!(from, (*from).global == (*to).global);
    api_check!(from, (*(*to).ci).top.offset_from((*to).top) >= n as isize);

    // Manual inline of lua_c_threadbarrier!(to) to bypass broken macros
    let marked = (*to).hdr.marked as i32;
    if (marked & (1 << BLACKBIT)) != 0 {
      lua_c_barrierback(to, to as *mut _, &mut (*to).gclist);
    }

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
