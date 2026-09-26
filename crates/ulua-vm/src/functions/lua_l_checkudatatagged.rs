use core::{ffi::c_void, str::from_utf8};

use crate::{
  functions::{
    cstr_bytes, lua_getuserdataname::lua_getuserdataname, lua_l_typeerror_l::lua_l_typeerror_l,
    lua_touserdatatagged::lua_touserdatatagged_ref,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证：`l` 为存活调用帧、`ud` 为可读实参栈槽、`tag` 落在 userdata 元表注册界内
/// （否则 `lua_touserdatatagged` 解引用 `udatametatable[tag]` 即 UB）；失配路径经 `l` 抛错、不返回空。cpp laux.cpp:140
pub(crate) unsafe fn lua_l_checkudatatagged(l: *mut LuaState, ud: i32, tag: i32) -> *mut c_void {
  // Safety: 契约保证 `l` 为存活调用帧、ud 索引可读且 tag 在 udatametatable 注册界内，失配路径抛错不返回
  unsafe {
    if let Some(p) = lua_touserdatatagged_ref(l, ud, tag) {
      return p as *mut c_void;
    }

    let tname = lua_getuserdataname(l, tag);
    let tname_str = from_utf8(cstr_bytes(tname)).unwrap_or("userdata");
    lua_l_typeerror_l(l, ud, tname_str);
  }
}

/// # Safety
/// 与 [`lua_l_checkudatatagged`] 同契约：`l` 存活、`ud` 栈槽可读、`tag` 在 userdata 元表注册界内；
/// C 边界入口，类型不符经 `l` 抛错并以 unwind 传播，调用方须处于受保护帧内。cpp laux.cpp:140
pub unsafe extern "C-unwind" fn lua_l_checkudatatagged_export(
  l: *mut LuaState,
  ud: i32,
  tag: i32,
) -> *mut c_void {
  // Safety: 直接转发同契约 `lua_l_checkudatatagged`；`l` 存活、tag 在注册类型数内、ud 索引可读
  unsafe { lua_l_checkudatatagged(l, ud, tag) }
}
