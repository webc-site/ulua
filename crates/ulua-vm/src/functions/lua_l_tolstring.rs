//! Node: `cxx:Function:Luau.VM:VM/src/laux.cpp:616:luaL_tolstring`
//! Source: `VM/src/laux.cpp:616-679` (hand-ported)

use core::{
  ffi::{CStr, c_char, c_int},
  ptr::null_mut,
};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_encodepointer::lua_encodepointer, lua_l_callmeta::lua_l_callmeta,
    lua_l_error_l::lua_l_error_l, lua_l_typename::lua_l_typename,
    lua_pushfstring_l::lua_pushfstring_l, lua_pushlstring::lua_pushlstring,
    lua_pushstring::lua_pushstring, lua_pushvalue::lua_pushvalue, lua_toboolean::lua_toboolean,
    lua_tointeger_64::lua_tointeger_64, lua_tolstring::lua_tolstring, lua_tonumberx::lua_tonumberx,
    lua_topointer::lua_topointer, lua_tovector::lua_tovector, lua_type::lua_type,
    luai_int_2_str::luai_int2str, luai_num_2_str::luai_num2str,
  },
  macros::{
    lua_vector_size::LUA_VECTOR_SIZE, luai_maxint_2_str::LUAI_MAXINT2STR,
    luai_maxnum_2_str::LUAI_MAXNUM2STR,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_tolstring(l: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char {
  unsafe {
    if lua_l_callmeta(l, idx, c"__tostring".as_ptr()) != 0 {
      let s = lua_tolstring(l, -1, len);
      if s.is_null() {
        lua_l_error_l(
          l,
          c"'__tostring' must return a string".as_ptr(),
          format_args!("'__tostring' must return a string"),
        );
      }
      return s;
    }

    match lua_type(l, idx) {
      x if x == LuaType::Nil as c_int => {
        lua_pushlstring(l, c"nil".as_ptr(), 3);
      }
      x if x == LuaType::Boolean as c_int => {
        lua_pushstring(
          l,
          if lua_toboolean(l, idx) != 0 {
            c"true".as_ptr()
          } else {
            c"false".as_ptr()
          },
        );
      }
      x if x == LuaType::Number as c_int => {
        let mut isnum = 0;
        let n = lua_tonumberx(l, idx, &mut isnum);
        let mut s = [0 as c_char; LUAI_MAXNUM2STR as usize];
        let e = luai_num2str(s.as_mut_ptr(), n);
        lua_pushlstring(l, s.as_ptr(), e.offset_from(s.as_ptr()) as usize);
      }
      x if x == LuaType::Vector as c_int => {
        let v = lua_tovector(l, idx);
        let mut s = [0 as c_char; (LUAI_MAXNUM2STR as usize) * (LUA_VECTOR_SIZE as usize)];
        let mut e = s.as_mut_ptr();
        for i in 0..LUA_VECTOR_SIZE {
          if i != 0 {
            *e = b',' as c_char;
            e = e.add(1);
            *e = b' ' as c_char;
            e = e.add(1);
          }
          e = luai_num2str(e, *v.add(i as usize) as f64);
        }
        lua_pushlstring(l, s.as_ptr(), e.offset_from(s.as_ptr()) as usize);
      }
      x if x == LuaType::String as c_int => {
        lua_pushvalue(l, idx);
      }
      x if x == LuaType::Integer as c_int => {
        let val = lua_tointeger_64(l, idx, null_mut());
        let mut s = [0 as c_char; LUAI_MAXINT2STR as usize];
        let e = luai_int2str(s.as_mut_ptr(), val);
        lua_pushlstring(l, s.as_ptr(), e.offset_from(s.as_ptr()) as usize);
      }
      _ => {
        let ptr = lua_topointer(l, idx);
        let enc = lua_encodepointer(l, ptr as usize);
        let name = CStr::from_ptr(lua_l_typename(l, idx)).to_string_lossy();
        lua_pushfstring_l(
          l,
          c"%s: 0x%016llx".as_ptr(),
          format_args!("{}: 0x{:016x}", name, enc),
        );
      }
    }

    lua_tolstring(l, -1, len)
  }
}
