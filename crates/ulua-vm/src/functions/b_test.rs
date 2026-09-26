use crate::{
  functions::{andaux::andaux, lua_pushboolean::lua_pushboolean},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe fn b_test(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 指向本次 binary32 C 函数调用的存活 LuaState，实参栈槽按索引可读、栈顶留有压入结果的 LUA_MINSTACK 余量
  unsafe {
    let r = andaux(l);
    lua_pushboolean(l, if r != 0 { 1 } else { 0 });
    1
  }
}

lua_lib_fn!(pub(crate) fn b_test, b_test_arm);
