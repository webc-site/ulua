use core::ffi::{c_char, c_int};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_l_checklstring::lua_l_checklstring, lua_type::lua_type},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_optlstring")]
pub unsafe fn lua_l_optlstring(
  l: *mut lua_State,
  narg: c_int,
  def: *const c_char,
  len: *mut usize,
) -> *const c_char {
  unsafe {
    let is_none_or_nil = lua_type(l, narg) <= (LuaType::Nil as c_int);

    if is_none_or_nil {
      if !len.is_null() {
        if !def.is_null() {
          let mut strlen: usize = 0;
          let mut p = def;
          while *p != 0 {
            strlen += 1;
            p = p.add(1);
          }
          *len = strlen;
        } else {
          *len = 0;
        }
      }
      def
    } else {
      lua_l_checklstring(l, narg, len)
    }
  }
}
