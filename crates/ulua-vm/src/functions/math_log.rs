use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  macros::lua_isnoneornil::lua_isnoneornil,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_l_checknumber(l,1)` 要求索引 1 存在且数值，索引 2 可选
/// （`lua_isnoneornil` 判空，缺省走自然对数）；`lua_pushnumber` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lmathlib.cpp:146
pub(crate) unsafe extern "C-unwind" fn math_log(l: *mut LuaState) -> i32 {
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
