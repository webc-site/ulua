use core::ffi::c_int;

use crate::{macros::cast_int::cast_int, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_gettop")]
pub unsafe fn lua_gettop(l: *mut lua_State) -> c_int {
  unsafe { cast_int!((*l).top.offset_from((*l).base)) }
}
