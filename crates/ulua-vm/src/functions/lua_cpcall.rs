use core::ffi::{c_int, c_void};

use crate::{
  functions::{f_ccall::f_ccall, lua_d_pcall::lua_d_pcall},
  macros::{api_check::api_check, savestack::savestack},
  records::c_call_s::CCallS,
  type_aliases::{lua_c_function::LuaCFunction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_cpcall(l: *mut lua_State, func: LuaCFunction, ud: *mut c_void) -> c_int {
  unsafe {
    api_check!(l, (*l).status == 0);
    api_check!(l, func.is_some());

    let mut c = CCallS { func, ud };

    lua_d_pcall(
      l,
      Some(f_ccall),
      &mut c as *mut CCallS as *mut c_void,
      savestack!(l, (*l).top) as isize,
      0,
    )
  }
}
