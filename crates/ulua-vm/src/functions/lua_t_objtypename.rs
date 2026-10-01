use core::ffi::c_char;

use crate::{
  functions::lua_t_objtypenamestr::lua_t_objtypenamestr, macros::getstr::getstr,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`o` 须为存活 `TValue` 的非空共享只读借用（转交
/// `lua_t_objtypenamestr` 只读类型 tag 与 gc 指针，对 userdata 还会读其 metatable/`name`
/// 字段，故该对象须仍在 GC 存活集内），返回其类型名 C 串指针。
/// cpp `ltm.cpp:178`。
pub(crate) unsafe fn lua_t_objtypename(l: *mut LuaState, o: &TValue) -> *const c_char {
  unsafe { getstr(lua_t_objtypenamestr(l, o)) }
}
