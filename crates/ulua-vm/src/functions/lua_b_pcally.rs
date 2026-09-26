use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkany::lua_l_checkany, lua_pcallyieldable::lua_pcallyieldable,
  },
  macros::{lua_lib_fn::lua_lib_fn, lua_multret::LUA_MULTRET},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于 pcall 的受保护帧：栈 1 号位为待调用函数（`lua_l_checkany` 校验非无值），
/// `lua_gettop` 定出实参数后 `lua_pcallyieldable` 建立新保护帧、可 yield/抛错。cpp/VM/src/lbaselib.cpp:285。
pub(crate) unsafe fn lua_b_pcally(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);

    lua_pcallyieldable(l, lua_gettop(l) - 1, LUA_MULTRET, 0)
  }
}

lua_lib_fn!(pub(crate) fn lua_b_pcally, lua_b_pcally_arm);
