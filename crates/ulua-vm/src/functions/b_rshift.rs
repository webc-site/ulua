use crate::{
  functions::{
    b_shift::b_shift, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe extern "C-unwind" fn b_rshift(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 指向本次 binary32 C 函数调用的存活 LuaState，实参栈槽按索引可读、栈顶留有压入结果的 LUA_MINSTACK 余量
  unsafe {
    // wrapping_neg avoids UB on INT_MIN (C++ negates a plain `int`); b_shift
    // treats the magnitude via unsigned_abs, so the wrapped value is handled.
    b_shift(
      l,
      lua_l_checkunsigned(l, 1),
      lua_l_checkinteger(l, 2).wrapping_neg(),
    )
  }
}
