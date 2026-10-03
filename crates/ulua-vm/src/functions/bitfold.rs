use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  macros::trim::trim,
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// binary32 n 元位折叠骨架（cpp lbitlib.cpp andaux 泛化）：实参 1..=n 经 `op` 逐个并入
/// `init`，结果 `trim` 收口。`op` 为各调用点单态化的闭包，无 dyn、无间接开销。
///
/// 调用序契约（正确性，非内存安全）：以 binary32 C 函数约定被调——所需实参按 API
/// 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错）；`l` 存活与独占由
/// `&mut LuaState` 承载。
pub(crate) fn bitfold(l: &mut LuaState, init: BUint, op: impl Fn(BUint, BUint) -> BUint) -> BUint {
  let n = l.get_top();
  let mut r = init;

  for i in 1..=n {
    r = op(r, lua_l_checkunsigned(l, i));
  }

  trim(r)
}

/// binary32 n 元位折叠入口骨架（band/bor/bxor 共用）：[`bitfold`] 折叠后压栈，返回 1。
/// `init` 取折叠单位元（AND=!0，OR/XOR=0）。
///
/// 调用序契约（正确性，非内存安全）：同 [`bitfold`]，另结果压栈需栈顶留 1 空槽。
#[inline]
pub(crate) fn bit_fold_push(
  l: &mut LuaState,
  init: BUint,
  op: impl Fn(BUint, BUint) -> BUint,
) -> i32 {
  let r = bitfold(l, init, op);
  lua_pushunsigned(l, r);
  1
}
