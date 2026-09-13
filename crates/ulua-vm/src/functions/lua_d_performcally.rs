use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_check_cstack::luaD_checkCstack, performcall::performcall},
  macros::{
    lua_c_check_gc::luaC_checkGC, lua_callinfo_opyield::LUA_CALLINFO_OPYIELD,
    luai_maxccalls::LUAI_MAXCCALLS, restoreci::restoreci, saveci::saveci,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_d_performcally")]
pub unsafe fn lua_d_performcally(l: *mut lua_State, func: StkId, nresults: c_int) -> bool {
  unsafe {
    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    if (*l).n_ccalls >= LUAI_MAXCCALLS as u16 {
      luaD_checkCstack(l);
    }

    (*l).base_ccalls = (*l).base_ccalls.wrapping_add(1);

    let cioffset = saveci!(l, (*l).ci);

    performcall(l, func, nresults, false);

    if (*l).status != LuaStatus::Ok as u8 {
      let caller = restoreci!(l, cioffset);
      (*caller).flags |= LUA_CALLINFO_OPYIELD as u32;
      return true;
    }

    (*l).base_ccalls = (*l).base_ccalls.wrapping_sub(1);
    (*l).n_ccalls = (*l).n_ccalls.wrapping_sub(1);
    luaC_checkGC!(l);
    false
  }
}
