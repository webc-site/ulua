use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  macros::lua_isnoneornil::lua_isnoneornil,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn math_log(l: *mut lua_State) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let res = if lua_isnoneornil!(l, 2) {
      x.ln()
    } else {
      let base = lua_l_checknumber(l, 2);
      if base == 2.0 {
        x.log2()
      } else if base == 10.0 {
        x.log10()
      } else {
        x.ln() / base.ln()
      }
    };

    lua_pushnumber(l, res);
    1
  }
}
