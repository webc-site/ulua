//! `ltable.cpp` 的 `luaG_runerror(L, "table overflow")` 平消息形态: 无格式化参数,
//! 直接 pusherror + throw。setnodevector / setarrayvector / resize 三处共用
//! (对应 `VM/src/ltable.cpp:396,417,452`)。

use core::ffi::{c_char, c_int};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::luaD_throw, lua_g_pusherror::lua_g_pusherror},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// `l` 必须指向存活的 `lua_State`; `msg` 必须是 NUL 结尾的有效 C 字符串。
pub(crate) unsafe fn runerror(l: *mut lua_State, msg: *const c_char) -> ! {
  unsafe {
    lua_g_pusherror(l, msg);
    luaD_throw(l, LuaStatus::ErrRun as c_int);
  }
}
