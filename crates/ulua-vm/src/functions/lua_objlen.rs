use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{index_2_addr::index_2_addr, lua_h_getn::lua_h_getn},
  macros::{bufvalue::bufvalue, hvalue::hvalue, tsvalue::tsvalue, ttype::ttype, uvalue::uvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_objlen(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    let o: StkId = index_2_addr(l, idx);
    let tt = ttype!(o);

    if tt == LuaType::String as i32 {
      (*tsvalue!(o)).len as i32
    } else if tt == LuaType::UserData as i32 {
      uvalue!(o).len as i32
    } else if tt == LuaType::Buffer as i32 {
      bufvalue!(o).len as i32
    } else if tt == LuaType::Table as i32 {
      lua_h_getn(hvalue!(o))
    } else {
      0
    }
  }
}
