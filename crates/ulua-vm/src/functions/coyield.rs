use core::ffi::c_int;

use crate::{
  functions::lua_yield::lua_yield, macros::cast_int::cast_int, type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_coyield"))]
pub(crate) unsafe extern "C-unwind" fn coyield(l: *mut lua_State) -> c_int {
  unsafe {
    let nres = cast_int!((*l).top.offset_from((*l).base));
    lua_yield(l, nres)
  }
}
