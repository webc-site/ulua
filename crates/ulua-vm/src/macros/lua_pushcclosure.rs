use core::ffi::c_char;

use crate::{
  functions::lua_pushcclosurek::lua_pushcclosurek, records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且栈顶恰有 `nup` 个待捕获上值（`0 <= nup <= upvalusage`）；
/// `f` 须为遵循 Lua C 函数约定的指针；`debugname` 须为空或指向闭包存活期内有效的 NUL 字符串。
/// cpp `lua.h:526` 宏：等价于 `lua_pushcclosurek(l, fn, debugname, nup, NULL)`，continuation
/// 传空，展开点不得要求该闭包可 yield。
#[inline(always)]
pub unsafe fn lua_pushcclosure(
  l: *mut LuaState,
  f: LuaCFunction,
  debugname: *const c_char,
  nup: i32,
) {
  // Safety: 契约保证 `l` 存活、f/debugname/nup 满足 pushcclosurek 前置，continuation 显式传 None
  unsafe {
    lua_pushcclosurek(l, f, debugname, nup, None);
  }
}
