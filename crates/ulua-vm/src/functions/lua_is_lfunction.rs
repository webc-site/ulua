use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::index_2_addr::index2addr,
  macros::{clvalue::clvalue, ttype::ttype},
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_is_lfunction(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if ttype!(o) == LuaType::Function as c_int && (*clvalue!(o)).is_c == 0 {
      1
    } else {
      0
    }
  }
}
