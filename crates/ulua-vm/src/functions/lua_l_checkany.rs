//! Source: `VM/src/laux.cpp:159-163` (hand-ported)

use crate::{
  enums::lua_type::LuaType, functions::lua_type::lua_type, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v43 收形后
/// 判型经安全门面 `lua_type`，体内无裸操作，仅抛错按 r16-v29 判例留一处 `as_mut_ptr` 窄重建，故降为安全
/// `fn`）：`l` 须处于可抛错的受保护帧：`narg` 为合法（伪）索引，`lua_type(l, narg)` 读该栈槽类型；
/// 若为 NONE（缺参）则经 `luaL_error` 抛 "missing argument"（可分配、经 unwind 回退）。正常返回时不写栈。
/// cpp VM/src/laux.cpp:170
pub(crate) fn lua_l_checkany(l: &mut LuaState, narg: i32) {
  if lua_type(l, narg) == LuaType::None as i32 {
    // SAFETY: `l` 为借用形式的存活调用帧（&mut 保证有效且独占），as_mut_ptr 由该借用重取裸指针，luaL_error 抛错不返回。
    unsafe { luaL_error!(l.as_mut_ptr(), "missing argument #{}", narg) };
  }
}
