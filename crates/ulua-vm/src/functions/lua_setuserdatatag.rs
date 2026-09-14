use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT, ttisuserdata::ttisuserdata},
  records::udata::Udata,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setuserdatatag(l: *mut lua_State, idx: c_int, tag: c_int) {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    let o: StkId = index2addr(l, idx);
    api_check!(l, ttisuserdata!(o));
    let u = core::ptr::addr_of_mut!((*(*o).value.gc).u) as *mut Udata;
    (*u).tag = tag as u8;
  }
}
