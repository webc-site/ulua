use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType, functions::lua_type::lua_type, type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_isstring")]
pub unsafe fn lua_isstring(l: *mut lua_State, idx: c_int) -> c_int {
  // SAFETY：lua_type 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let t = unsafe { lua_type(l, idx) };
  if t == LuaType::String as c_int || t == LuaType::Number as c_int {
    1
  } else {
    0
  }
}
