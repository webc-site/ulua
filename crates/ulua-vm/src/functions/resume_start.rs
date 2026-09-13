use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_c_barrierback::lua_c_barrierback, resume_error::resume_error},
  macros::{api_check::api_check, isblack::isblack, luai_maxccalls::LUAI_MAXCCALLS},
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn resume_start(l: *mut lua_State, from: *mut lua_State, nargs: c_int) -> c_int {
  unsafe {
    api_check!(l, nargs >= 0);
    api_check!(l, (*l).top.offset_from((*l).base) >= nargs as isize);

    if (*l).status != LuaStatus::Yield as u8
      && (*l).status != LuaStatus::Break as u8
      && ((*l).status != 0 || (*l).ci != (*l).base_ci)
    {
      return resume_error(l, c"cannot resume non-suspended coroutine".as_ptr(), nargs);
    }

    (*l).n_ccalls = if !from.is_null() { (*from).n_ccalls } else { 0 };
    if (*l).n_ccalls as i32 >= LUAI_MAXCCALLS {
      return resume_error(l, c"C stack overflow".as_ptr(), nargs);
    }

    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    (*l).base_ccalls = (*l).n_ccalls;
    (*l).isactive = true;

    let o = l as *mut GCObject;
    if isblack!(o) {
      lua_c_barrierback(l, o, core::ptr::addr_of_mut!((*l).gclist));
    }

    LuaStatus::Ok as c_int
  }
}
