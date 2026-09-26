use core::ffi::c_char;

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::lua_h_getstr,
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT, svalue::svalue},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub(crate) unsafe fn lua_getuserdataname(l: *mut LuaState, tag: i32) -> *const c_char {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = (*(*l).global).udatamt[tag as usize];
    if !mt.is_null() {
      let type_ = lua_h_getstr(mt, (*(*l).global).tmname[TMS::TmType as usize]);
      if (*type_).is_string() {
        return svalue!(type_);
      }
    }

    c"userdata".as_ptr()
  }
}

/// # Safety
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe extern "C-unwind" fn lua_getuserdataname_export(
  l: *mut LuaState,
  tag: i32,
) -> *const c_char {
  // Safety: 导出壳原样转发同契约 `lua_getuserdataname`；tag 在 udatametatable 注册界内
  unsafe { lua_getuserdataname(l, tag) }
}
