use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    auxwrapcont::auxwrapcont, auxwrapy::auxwrapy, cocreate::cocreate,
    lua_pushcclosurek::lua_pushcclosurek,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_cowrap"))]
pub(crate) unsafe extern "C-unwind" fn cowrap(l: *mut lua_State) -> c_int {
  unsafe {
    cocreate(l);
    lua_pushcclosurek(l, Some(auxwrapy), null(), 1, Some(auxwrapcont));
    1
  }
}
