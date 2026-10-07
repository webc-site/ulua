use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_check_cstack::lua_d_check_cstack, performcall::performcall},
  macros::{
    lua_c_check_gc::lua_c_check_gc, lua_callinfo_opyield::LUA_CALLINFO_OPYIELD,
    luai_maxccalls::LUAI_MAXCCALLS, restoreci::restoreci, saveci::saveci,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_performcally(l: *mut LuaState, func: StkId, nresults: i32) -> bool {
  unsafe {
    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    if (*l).n_ccalls >= LUAI_MAXCCALLS as u16 {
      lua_d_check_cstack(l);
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
    lua_c_check_gc!(l);
    false
  }
}
