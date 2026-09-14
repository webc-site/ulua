use core::ffi::{c_int, c_void};

use ulua_common::FFlag;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_rawrunprotected_ldo::luaD_rawrunprotected, lua_d_seterrorobj::luaD_seterrorobj,
    lua_isyieldable::lua_isyieldable, resume_findhandler::resume_findhandler,
    resume_handle::resume_handle,
  },
  macros::expandstacklimit::expandstacklimit,
  records::call_info::CallInfo,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn resume_finish(
  l: *mut lua_State,
  mut status: c_int,
  old_n_ccalls: c_int,
) -> c_int {
  unsafe {
    let mut ch: *mut CallInfo;

    loop {
      ch = resume_findhandler(l);
      if status == LuaStatus::Ok as c_int || ch.is_null() {
        break;
      }

      if lua_isyieldable(l) != 0
        && let Some(debugprotectederror) = (*(*l).global).cb.debugprotectederror
      {
        debugprotectederror(l);

        if (*l).status == LuaStatus::Break as u8 {
          status = LuaStatus::Ok as c_int;
          break;
        }
      }

      if FFlag::LuauXpcallFixMessageYieldPath.get() {
        (*l).base_ccalls = old_n_ccalls as u16;
      } else {
        (*l).n_ccalls = old_n_ccalls as u16;
        (*l).base_ccalls = (*l).n_ccalls;
      }

      (*l).status = status as u8;
      status = luaD_rawrunprotected(l, Some(resume_handle), ch as *mut c_void);
    }

    if FFlag::LuauResumeRestoreCcalls.get() {
      (*l).n_ccalls = (old_n_ccalls - 1) as u16;
    } else {
      (*l).base_ccalls = (*l).base_ccalls.wrapping_sub(1);
      (*l).n_ccalls = (*l).base_ccalls;
    }

    (*l).base_ccalls = (*l).n_ccalls;
    (*l).isactive = false;

    if status != LuaStatus::Ok as c_int {
      (*l).status = status as u8;
      luaD_seterrorobj(l, status, (*l).top);
      (*(*l).ci).top = (*l).top;
    } else if (*l).status == LuaStatus::Ok as u8 {
      expandstacklimit!(l, (*l).top);
    }

    (*l).status as c_int
  }
}
