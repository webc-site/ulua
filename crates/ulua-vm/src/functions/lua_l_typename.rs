use core::{ffi::c_char, ptr::eq};

use crate::{
  functions::{
    lua_a_toobject::lua_a_toobject, lua_t_objtypename::lua_t_objtypename,
    lua_typename::NO_VALUE_BYTES,
  },
  macros::lua_o_nilobject::LUA_O_NILOBJECT,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引；`lua_a_toobject` 返回值允许为 NULL 或等于
/// `LUA_O_NILOBJECT`（此时返回静态串 "no value"），否则须为指向存活 TValue 的指针（交给 `luaT_objtypename` 读类型）。
/// cpp `laux.cpp:371`。
pub unsafe fn lua_l_typename(l: *mut LuaState, idx: i32) -> *const c_char {
  unsafe {
    let obj: *const TValue = lua_a_toobject(l, idx);

    if obj.is_null() || eq(obj, LUA_O_NILOBJECT) {
      NO_VALUE_BYTES.as_ptr().cast()
    } else {
      lua_t_objtypename(l, obj)
    }
  }
}
