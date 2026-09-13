//! Node: `cxx:Function:Luau.VM:VM/src/lvmexecute.cpp:147:luau_callhook`
//! Source: `VM/src/lvmexecute.cpp:147-200` (hand-ported)

use core::{ffi::c_void, mem::zeroed};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_g_getline::luaG_getline,
  macros::{
    clvalue::clvalue, lua_d_checkstack::luaD_checkstack, lua_minstack::LUA_MINSTACK, pc_rel::pcRel,
    restorestack::restorestack, savestack::savestack,
  },
  records::lua_debug::LuaDebug,
  type_aliases::{lua_hook::LuaHook, lua_state::lua_State},
};

/// C++ `LUAU_NOINLINE void luau_callhook(lua_State* l, lua_Hook hook, void* userdata)`.
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(never)]
pub unsafe fn luau_callhook(l: *mut lua_State, hook: LuaHook, userdata: *mut c_void) {
  unsafe {
    let base = savestack!(l, (*l).base);
    let top = savestack!(l, (*l).top);
    let ci_top = savestack!(l, (*(*l).ci).top);
    let status = (*l).status;

    // if the hook is called externally on a paused thread, we need to make
    // sure the paused thread can emit Luau calls
    if status == LuaStatus::Yield as u8 || status == LuaStatus::Break as u8 {
      (*l).status = 0;
      (*l).base = (*(*l).ci).base;
    }

    let cl = clvalue!((*(*l).ci).func);

    // note: the pc expectations of the hook are matching the general "pc
    // points to next instruction"; however, for the hook to be able to
    // continue execution from the same point, this is called with savedpc at
    // the *current* instruction. this needs to be called before
    // luaD_checkstack in case it fails to reallocate stack
    let oldsavedpc = (*(*l).ci).savedpc;

    if !(*(*l).ci).savedpc.is_null() {
      let code_end = {
        let l = &(*cl).inner.l;
        (*l.p).code.add((*l.p).sizecode as usize)
      };
      if (*(*l).ci).savedpc != code_end {
        (*(*l).ci).savedpc = (*(*l).ci).savedpc.add(1);
      }
    }

    luaD_checkstack!(l, LUA_MINSTACK); // ensure minimum stack size
    (*(*l).ci).top = (*l).top.add(LUA_MINSTACK as usize);
    LUAU_ASSERT!((*(*l).ci).top <= (*l).stack_last);

    let mut ar: LuaDebug = zeroed();
    ar.currentline = if (*cl).is_c != 0 {
      -1
    } else {
      let p = {
        let l = &(*cl).inner.l;
        l.p
      };
      luaG_getline(p, pcRel!((*(*l).ci).savedpc, p))
    };
    ar.userdata = userdata;

    if let Some(hook) = hook {
      hook(l, &mut ar);
    }

    (*(*l).ci).savedpc = oldsavedpc;

    (*(*l).ci).top = restorestack!(l, ci_top);
    (*l).top = restorestack!(l, top);

    // note that we only restore the paused state if the hook hasn't yielded by itself
    if status == LuaStatus::Yield as u8 && (*l).status != LuaStatus::Yield as u8 {
      (*l).status = LuaStatus::Yield as u8;
      (*l).base = restorestack!(l, base);
    } else if status == LuaStatus::Break as u8 {
      LUAU_ASSERT!((*l).status != LuaStatus::Break as u8); // hook shouldn't break again

      (*l).status = LuaStatus::Break as u8;
      (*l).base = restorestack!(l, base);
    }
  }
}
