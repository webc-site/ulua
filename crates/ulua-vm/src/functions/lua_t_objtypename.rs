use core::ffi::c_char;

use crate::{
  functions::lua_t_objtypenamestr::lua_t_objtypenamestr, macros::getstr::getstr,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`o` 须为指向存活 TValue 的非空指针（转交 `luaT_objtypenamestr` 读类型，
/// 对 userdata 还会读其 metatable/`name` 字段，故该对象须仍在 GC 存活集内），返回其类型名 C 串指针。
/// cpp `ltm.cpp:178`。
pub(crate) unsafe fn lua_t_objtypename(l: *mut LuaState, o: *const TValue) -> *const c_char {
  unsafe { getstr(lua_t_objtypenamestr(l, o)) }
}
