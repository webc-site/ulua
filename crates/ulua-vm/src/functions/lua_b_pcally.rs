use crate::{
  functions::lua_pcallyieldable::lua_pcallyieldable,
  macros::{lua_lib_fn::lua_lib_fn, lua_multret::LUA_MULTRET},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`lua_pcallyieldable` 仍收裸形，
/// 转手经一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句），屏障按 r16-v21 判例保留；`l` 仍须处于
/// pcall 的受保护帧：栈 1 号位为待调用函数（`check_any` 校验非无值），`get_top` 定出实参数后
/// `lua_pcallyieldable` 建立新保护帧、可 yield/抛错。cpp/VM/src/lbaselib.cpp:285。
pub(crate) unsafe fn lua_b_pcally(l: &mut LuaState) -> i32 {
  unsafe {
    l.check_any(1);

    lua_pcallyieldable(l.as_mut_ptr(), l.get_top() - 1, LUA_MULTRET, 0)
  }
}

lua_lib_fn!(pub(crate) fn lua_b_pcally @ref, lua_b_pcally_arm);
