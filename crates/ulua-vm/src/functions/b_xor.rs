use crate::{
  functions::bitfold::bit_fold_push, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// binary32 `bxor` 核心：单位元 0 经 [`bit_fold_push`] 折叠、结果纯数值压栈。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调（binary32 C 函数面），
/// 实参越界或非数值由 check*/argerror 报错，结果压栈需栈顶留 1 空槽；`l` 存活与
/// 独占由 `&mut LuaState` 承载。
pub(crate) fn b_xor(l: &mut LuaState) -> i32 {
  bit_fold_push(l, 0 as BUint, |a, b| a ^ b)
}

lua_lib_fn!(pub(crate) fn b_xor @ref, b_xor_arm);
