use core::{ffi::c_char, ptr::null};

use crate::{
  macros::{api_check::api_check, getstr::getstr, lua_lutag_limit::LUA_LUTAG_LIMIT},
  records::{lua_state::LuaState, t_string::tstring},
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_getlightuserdataname(l: *mut LuaState, tag: i32) -> *const c_char {
  api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);

  // Safety: 契约保证 `l` 的 global 存活且 tag 在 light userdatum 名称注册界内，读回登记时的 C 串指针
  unsafe {
    let global = (*l).global;
    let name = (*global).lightuserdataname[tag as usize];
    if name.is_null() {
      null()
    } else {
      getstr(name as *const tstring)
    }
  }
}
