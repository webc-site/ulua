use core::ffi::c_char;

use crate::{
  enums::tms::TMS,
  functions::{cstr, lua_h_getstr::lua_h_getstr},
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
      // B2-2a 任务B：同借用窗口内即时读判定——Option<Slot> 原生收口，svalue! 宏
      // 边界直收 `.get()` 只读视图（展开为对 &TValue 的解引用，读形不变）
      if let Some(type_) = lua_h_getstr(&*mt, (*(*l).global).tmname[TMS::TmType as usize])
        && type_.get().is_string()
      {
        return svalue!(type_.get());
      }
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
