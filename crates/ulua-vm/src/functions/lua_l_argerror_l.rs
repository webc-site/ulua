use crate::{
  functions::currfuncname::currfuncname, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`currfuncname(l)` 读当前帧函数名（可空走无函数名分支）；
/// `narg` 为出错的栈参数序号（仅用于文案，可正可负但须为调用方约定的实参位）；`extramsg` 为 &str 无越界风险。
/// 经 `luaL_error` 抛错，返回 `!`（永不正常返回）。
/// cpp VM/src/laux.cpp:35
pub unsafe fn lua_l_argerror_l(l: *mut LuaState, narg: i32, extramsg: &str) -> ! {
  // Safety: 契约保证 l 为存活受保护帧，currfuncname 只读帧名、luaL_error 抛错不返回
  unsafe {
    match currfuncname(l) {
      Some(fname) => luaL_error!(
        l,
        "invalid argument #{} to '{}' ({})",
        narg,
        String::from_utf8_lossy(fname),
        extramsg
      ),
      None => luaL_error!(l, "invalid argument #{} ({})", narg, extramsg),
    }
  }
}
