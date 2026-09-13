use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_d_check_cstack::luaD_checkCstack, performcall::performcall},
  macros::{
    isyielded::isyielded, lua_c_check_gc::luaC_checkGC, lua_multret::LUA_MULTRET,
    luai_maxccalls::LUAI_MAXCCALLS, restorestack::restorestack, savestack::savestack,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

#[unsafe(export_name = "ulua_lua_d_callny")]
pub(crate) unsafe fn lua_d_callny(l: *mut lua_State, func: StkId, nresults: c_int) {
  unsafe {
    let l_ref = &mut *l;

    l_ref.n_ccalls += 1;
    if l_ref.n_ccalls >= LUAI_MAXCCALLS as u16 {
      luaD_checkCstack(l);
    }

    LUAU_ASSERT!(l_ref.n_ccalls > l_ref.base_ccalls);

    let funcoffset = savestack!(l, func);

    performcall(l, func, nresults, false);

    LUAU_ASSERT!(!isyielded(l));

    if nresults != LUA_MULTRET {
      (*l).top = restorestack!(l, funcoffset).add(nresults as usize);
    }

    (*l).n_ccalls -= 1;
    luaC_checkGC!(l);
  }
}
