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

/// `luaL_typename` 核心（cpp `laux.cpp:371`）。`l` 以引用传入（存活由类型保证）；
/// `idx` 为任意（伪）索引，越界经 `lua_a_toobject` 折叠为 NULL（返回静态串
/// "no value"，与 cpp 越界分支一致）。仅读槽类型名，不写栈、不分配、不抛错。
pub fn lua_l_typename(l: &LuaState, idx: i32) -> *const c_char {
  let obj: *const TValue = lua_a_toobject(l, idx);

  if obj.is_null() || eq(obj, LUA_O_NILOBJECT) {
    NO_VALUE_BYTES.as_ptr().cast()
  } else {
    // SAFETY:obj 指向存活 TValue；`lua_t_objtypename` 只读类型名表，不写穿 `l`。
    unsafe { lua_t_objtypename(l.read_ptr(), &*obj) }
  }
}
