use crate::{
  functions::{b_rot::b_rot, lua_l_checkinteger::lua_l_checkinteger},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe fn b_lrot(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧，`b_rot` 与 `lua_l_checkinteger` 仅需该前置（实参 2 在栈上可读）
  unsafe { b_rot(l, lua_l_checkinteger(l, 2)) }
}

lua_lib_fn!(pub(crate) fn b_lrot, b_lrot_arm);
