use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// 单目 bit32 库函数骨架：取 1 号无符号实参，套 `f` 后压回，返回 1。
///
/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
#[inline]
pub(crate) unsafe fn bit_map1(l: *mut LuaState, f: impl Fn(BUint) -> BUint) -> i32 {
  // Safety: 契约保证 `l` 指向本次 binary32 C 函数调用的存活 LuaState，实参栈槽按索引可读、栈顶留有压入结果的 LUA_MINSTACK 余量
  unsafe {
    let v = lua_l_checkunsigned(l, 1) as BUint;
    lua_pushunsigned(l, f(v));
    1
  }
}
