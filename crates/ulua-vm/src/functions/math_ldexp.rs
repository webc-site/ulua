use crate::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checknumber::lua_l_checknumber,
    lua_pushnumber::lua_pushnumber,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于本 C 函数受保护帧：栈 1 号位经 `lua_l_checknumber` 取数字、2 号位经 `lua_l_checkinteger`
/// 取整数指数（非对应类型抛错），结果经 `lua_pushnumber` 写回。cpp/VM/src/lmathlib.cpp:198 math_ldexp。
pub unsafe extern "C-unwind" fn math_ldexp(l: *mut LuaState) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let exp = lua_l_checkinteger(l, 2);
    // ldexp(x, exp) is x * 2^exp
    lua_pushnumber(l, x * (2.0f64).powi(exp));
    1
  }
}
