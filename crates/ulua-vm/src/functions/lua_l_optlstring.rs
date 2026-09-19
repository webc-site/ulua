use core::ffi::{CStr, c_char, c_int};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_l_checklstring::lua_l_checklstring, lua_type::lua_type},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_l_optlstring"))]
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
        // C++ `def ? strlen(def) : 0`：CStr 零拷贝求长度，与 strlen 语义一致
        *len = if !def.is_null() {
          CStr::from_ptr(def).to_bytes().len()
        } else {
          0
        };
      }
      def
    } else {
      lua_l_checklstring(l, narg, len)
    }
  }
}
