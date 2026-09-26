use core::ffi::c_void;

use crate::{
  functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged, records::lua_state::LuaState,
};

/// cpp `lua.h:519` `#define lua_pushlightuserdata(L, p) lua_pushlightuserdatatagged(L, p, 0)` 对应。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`。
#[inline(always)]
pub unsafe fn lua_pushlightuserdata(l: *mut LuaState, p: *mut c_void) {
  // Safety: 契约保证 `l` 存活；`p` 仅作不透明值存入 lightuserdata 槽，本函数不解引用
  unsafe { lua_pushlightuserdatatagged(l, p, 0) };
}
