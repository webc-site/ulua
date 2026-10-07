use core::{ffi::c_void, str::from_utf8};

use crate::{
  functions::{
    lua_getuserdataname::lua_getuserdataname_bytes, lua_l_typeerror_l::lua_l_typeerror_l,
    lua_touserdatatagged::lua_touserdatatagged_ref,
  },
  records::lua_state::LuaState,
};

const DEFAULT_USERDATA_TYPE: &str = "userdata";

/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 承载，形制对齐同族
/// 孪生 `lua_l_checkudata`）：`ud` 为可读实参栈槽、`tag` 落在 userdata 元表注册界内（否则
/// [`lua_touserdatatagged_ref`] 解引用 `udatamt[tag]` 即 UB）；失配路径经 `l` 抛错、发散不返回。
/// cpp laux.cpp:140
pub(crate) fn lua_l_checkudatatagged(l: &mut LuaState, ud: i32, tag: i32) -> *mut c_void {
  // SAFETY: 契约由上文承载——`ud` 栈槽可读、`tag` 界内；两枚转调核心借出的引用只在当句内使用，
  // 且 `&mut LuaState` 的独占保证它们不与本帧其它别名交叠；失配路径 `lua_l_typeerror_l` 抛错
  // （返回 `!`），故本函数要么返回界内数据指针、要么不返回
  unsafe {
    if let Some(p) = lua_touserdatatagged_ref(l, ud, tag) {
      return p as *mut c_void;
    }

    let tname = lua_getuserdataname_bytes(l, tag);
    let tname_str = from_utf8(tname).unwrap_or(DEFAULT_USERDATA_TYPE);
    lua_l_typeerror_l(l, ud, tname_str);
  }
}

/// # Safety
/// C ABI 边界臂：`l` 须为指向存活 `LuaState` 的非空指针，`ud` 为可读实参栈槽、`tag` 在 userdata
/// 元表注册界内（[`lua_l_checkudatatagged`] 同契约）；类型不符经 `l` 抛错并以 unwind 传播，
/// 调用方须处于受保护帧内。cpp laux.cpp:140
pub unsafe extern "C-unwind" fn lua_l_checkudatatagged_export(
  l: *mut LuaState,
  ud: i32,
  tag: i32,
) -> *mut c_void {
  // SAFETY: 契约保证 `l` 非空且指向存活 `LuaState`；`&mut *l` 一次性重借用即收形后核心期望的
  // 接收者形，本帧不再解引用该指针；返回值指向该 userdata 的数据块，在其所属对象存活期内可读
  unsafe { lua_l_checkudatatagged(&mut *l, ud, tag) }
}
