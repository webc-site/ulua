use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_call::lua_call,
  macros::{
    api_check::api_check, c_call_yield::C_CALL_YIELD, clvalue::clvalue, iscfunction::iscfunction,
    isyielded::isyielded,
  },
  records::closure::CClosure,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_callyieldable(l: *mut lua_State, nargs: c_int, nresults: c_int) -> c_int {
  unsafe {
    api_check!(l, iscfunction!((*(*l).ci).func));
    let cl = clvalue!((*(*l).ci).func);
    let c = core::ptr::addr_of!((*cl).inner.c).cast::<CClosure>();
    api_check!(l, (*c).cont.is_some());

    lua_call(l, nargs, nresults);

    if isyielded(l) {
      return C_CALL_YIELD;
    }

    ((*c).cont.unwrap())(l, LuaStatus::Ok as c_int)
  }
}
