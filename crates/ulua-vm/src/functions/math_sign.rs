use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_math_sign"))]
pub(crate) unsafe extern "C-unwind" fn math_sign(l: *mut lua_State) -> i32 {
  unsafe {
    let v = lua_l_checknumber(l, 1);
    let res = if v > 0.0 {
      1.0
    } else if v < 0.0 {
      -1.0
    } else {
      0.0
    };

    lua_pushnumber(l, res);
    1
  }
}
