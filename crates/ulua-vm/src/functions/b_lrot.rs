use crate::{
  functions::b_rot::b_rot, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证本次 binary32 C 函数调用的实参 2 按 API 索引约定位于栈上可读
/// （越界或非数值由 check*/argerror 报错路径抛出）、栈顶预留结果空间；
/// `b_rot` 与 `lua_l_checkinteger` 仅需该前置。
pub(crate) fn b_lrot(l: &mut LuaState) -> i32 {
  let i = l.check_integer(2);
  b_rot(l, i)
}

lua_lib_fn!(pub(crate) fn b_lrot @ref, b_lrot_arm);
