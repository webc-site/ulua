use crate::{
  functions::lua_l_checkinteger_64::lua_l_checkinteger_64, macros::lua_l_opt::luaL_opt,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_optinteger_64(l: *mut LuaState, narg: i32, def: i64) -> i64 {
  unsafe {
    // 对应 cpp laux.h 的 luaL_opt 宏：nil/缺省取 def，否则调用 check 版本
    luaL_opt!(l, lua_l_checkinteger_64, narg, def)
  }
}
