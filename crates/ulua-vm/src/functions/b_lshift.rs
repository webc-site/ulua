use crate::{
  functions::{
    b_shift::b_shift, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe fn b_lshift(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧，`b_shift` 与 `checkunsigned/checkinteger` 仅需该前置（实参 1/2 在栈上可读）
  unsafe { b_shift(l, lua_l_checkunsigned(l, 1), lua_l_checkinteger(l, 2)) }
}

lua_lib_fn!(pub(crate) fn b_lshift, b_lshift_arm);
