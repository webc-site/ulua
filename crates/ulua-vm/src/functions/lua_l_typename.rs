use core::{
  ffi::{c_char, c_int},
  ptr::eq,
};

use crate::{
  functions::{lua_a_toobject::luaA_toobject, lua_t_objtypename::lua_t_objtypename},
  macros::lua_o_nilobject::luaO_nilobject,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_typename(l: *mut lua_State, idx: c_int) -> *const c_char {
  unsafe {
    let obj: *const TValue = luaA_toobject(l, idx);

    if obj.is_null() || eq(obj, luaO_nilobject) {
      b"no value\0" as *const u8 as *const c_char
    } else {
      lua_t_objtypename(l, obj)
    }
  }
}
