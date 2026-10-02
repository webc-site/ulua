use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  enums::tms::TMS,
  functions::{cstr, lua_h_getstr::lua_h_getstr},
  macros::{api_check::api_check, getstr::getstr, lua_utag_limit::LUA_UTAG_LIMIT, svalue::svalue},
  records::lua_state::LuaState,
};

const USERDATA_NAME: &[u8] = b"userdata";

/// 字节切片形态获取 userdata 类型名称。
/// # Safety
/// `l` 必须指向存活 `LuaState`，tag 必须落在合法 tag 范围内。
pub(crate) unsafe fn lua_getuserdataname_bytes<'a>(l: *mut LuaState, tag: i32) -> &'a [u8] {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = (*l).gs_ref().udatamt[tag as usize];
    if !mt.is_null()
      && let Some(type_) = lua_h_getstr(&*mt, (*l).gs_ref().tmname[TMS::TmType as usize])
      && type_.get().is_string()
    {
      let ts = type_.get().as_string_ptr();
      return from_raw_parts(getstr(ts).cast::<u8>(), (*ts).len as usize);
    }

    USERDATA_NAME
  }
}

/// # Safety
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub(crate) unsafe fn lua_getuserdataname(l: *mut LuaState, tag: i32) -> *const c_char {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = (*l).gs_ref().udatamt[tag as usize];
    if !mt.is_null()
      && let Some(type_) = lua_h_getstr(&*mt, (*l).gs_ref().tmname[TMS::TmType as usize])
      && type_.get().is_string()
    {
      return svalue!(type_.get());
    }

    cstr(b"userdata\0")
  }
}

/// # Safety
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe extern "C-unwind" fn lua_getuserdataname_export(
  l: *mut LuaState,
  tag: i32,
) -> *const c_char {
  // SAFETY: 导出壳原样转发同契约 `lua_getuserdataname`；tag 在 udatametatable 注册界内
  unsafe { lua_getuserdataname(l, tag) }
}
