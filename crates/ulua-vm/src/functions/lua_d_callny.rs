use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_d_check_cstack::lua_d_check_cstack, performcall::performcall},
  macros::{
    isyielded::isyielded, lua_c_check_gc::lua_c_check_gc, lua_multret::LUA_MULTRET,
    luai_maxccalls::LUAI_MAXCCALLS, restorestack::restorestack, savestack::savestack,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 且不可 yield（call-with-no-yield）：`(*l).n_ccalls`/`base_ccalls` 满足 C 调用深度约束，
/// `func` 须为栈内合法 StkId（`savestack!`/`restorestack!` 记录并可跨栈重分配还原），`nresults` 为约定值或 `LUA_MULTRET`；
/// `performcall`/`luaC_checkGC` 可分配/GC/抛错，须在受保护帧内。cpp/VM/src/ldo.cpp:388 luaD_callny。
pub unsafe fn lua_d_callny(l: *mut LuaState, func: StkId, nresults: i32) {
  unsafe {
    // performcall/lua_d_check_cstack 会经裸指针写 *l，不能在其间持有长生命周期
    // &mut（与 lua_d_callint 家族的裸指针访问风格一致）
    (*l).n_ccalls += 1;
    if (*l).n_ccalls >= LUAI_MAXCCALLS as u16 {
      lua_d_check_cstack(l);
    }

    LUAU_ASSERT!((*l).n_ccalls > (*l).base_ccalls);

    let funcoffset = savestack!(l, func);

    performcall(l, func, nresults, false);

    LUAU_ASSERT!(!isyielded(&*l));

    if nresults != LUA_MULTRET {
      (*l).top = restorestack!(l, funcoffset).add(nresults as usize);
    }

    (*l).n_ccalls -= 1;
    lua_c_check_gc!(l);
  }
}
