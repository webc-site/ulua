use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkunsigned::lua_l_checkunsigned,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::trim::trim,
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// binary32 n 元位折叠骨架（cpp lbitlib.cpp andaux 泛化）：实参 1..=n 经 `op` 逐个并入
/// `init`，结果 `trim` 收口。`op` 为各调用点单态化的闭包，无 dyn、无间接开销。
///
/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错）。
pub(crate) unsafe fn bitfold(
  l: *mut LuaState,
  init: BUint,
  op: impl Fn(BUint, BUint) -> BUint,
) -> BUint {
  // Safety: 契约保证 `l` 指向本次 binary32 C 函数调用的存活 LuaState，实参栈槽按索引可读，折叠为纯数值运算无指针访问
  unsafe {
    let n = lua_gettop(l);
    let mut r = init;

    for i in 1..=n {
      r = op(r, lua_l_checkunsigned(l, i));
    }

    trim(r)
  }
}

/// binary32 n 元位折叠入口骨架（band/bor/bxor 共用）：[`bitfold`] 折叠后压栈，返回 1。
/// `init` 取折叠单位元（AND=!0，OR/XOR=0）。
///
/// # Safety
///
/// 同 [`bitfold`]。
#[inline]
pub(crate) unsafe fn bit_fold_push(
  l: *mut LuaState,
  init: BUint,
  op: impl Fn(BUint, BUint) -> BUint,
) -> i32 {
  // Safety: 契约同 bitfold——`l` 为存活调用帧，折叠为纯数值运算，栈顶已留压栈余量
  unsafe {
    let r = bitfold(l, init, op);
    lua_pushunsigned(l, r);
    1
  }
}
