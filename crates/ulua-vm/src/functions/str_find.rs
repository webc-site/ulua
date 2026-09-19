use core::ffi::c_int;

use crate::{functions::str_find_aux::str_find_aux, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_str_find"))]
pub(crate) unsafe extern "C-unwind" fn str_find(l: *mut lua_State) -> c_int {
  unsafe { str_find_aux(l, 1) }
}
