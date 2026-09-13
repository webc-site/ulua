use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_throw_ldo::luaD_throw, lua_g_pusherror::lua_g_pusherror, luau_poscall::luau_poscall,
    luau_precall::luau_precall, resume_continue::resume_continue,
  },
  macros::{
    curr_func::curr_func, isyielded::isyielded, lua_callinfo_return::LUA_CALLINFO_RETURN,
    lua_multret::LUA_MULTRET, pcrlua::PCRLUA, scheduled_reentry::SCHEDULED_REENTRY,
  },
  records::closure::CClosure,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

pub(crate) unsafe extern "C-unwind" fn resume(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let mut first_arg = ud as StkId;

    if (*l).status == LuaStatus::Ok as u8 {
      LUAU_ASSERT!((*l).ci == (*l).base_ci && first_arg >= (*l).base);
      if first_arg == (*l).base {
        lua_g_pusherror(l, c"cannot resume dead coroutine".as_ptr());
        luaD_throw(l, LuaStatus::ErrRun as i32);
      }

      let precallresult = luau_precall(l, first_arg.offset(-1), LUA_MULTRET);

      if (*l).status == SCHEDULED_REENTRY as u8 {
        first_arg = (*l).base;
      } else {
        if precallresult != PCRLUA {
          return;
        }

        (*(*l).ci).flags |= LUA_CALLINFO_RETURN as u32;
      }
    }

    if (*l).status != LuaStatus::Ok as u8 {
      LUAU_ASSERT!(first_arg >= (*l).base);
      LUAU_ASSERT!(isyielded(l));
      (*l).status = LuaStatus::Ok as u8;

      let cl = curr_func!(l);

      if (*cl).is_c != 0 {
        let c = core::ptr::addr_of!((*cl).inner.c).cast::<CClosure>();
        if (*c).cont.is_none() {
          luau_poscall(l, first_arg);
        } else {
          (*l).base = (*(*l).ci).base;
        }
      } else {
        (*l).base = (*(*l).ci).base;
      }
    }

    resume_continue(l);
  }
}
