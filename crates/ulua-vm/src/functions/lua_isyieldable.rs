use core::ffi::c_int;

use crate::type_aliases::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_isyieldable")]
pub unsafe fn lua_isyieldable(l: *mut lua_State) -> c_int {
  unsafe {
    if (*l).n_ccalls <= (*l).base_ccalls {
      1
    } else {
      0
    }
  }
}
