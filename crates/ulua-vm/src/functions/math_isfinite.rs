use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_math_isfinite"))]
pub(crate) unsafe extern "C-unwind" fn math_isfinite(l: *mut lua_State) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    lua_pushboolean(l, x.is_finite() as i32);
    1
  }
}
