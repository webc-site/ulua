use crate::{functions::lua_settop::lua_settop, records::lua_state::LuaState};

/// 从 Lua 栈中弹出 `n` 个元素。
///
/// 等价于 `lua_settop(l, -n - 1)`（对应 C/C++ 宏 `#define lua_pop(L,n) lua_settop(L, -(n)-1)`）。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`；`n` 须为不超过当前栈元素数的出栈个数。
#[inline]
pub unsafe fn lua_pop(l: *mut LuaState, n: i32) {
  // Safety: 契约保证 `l` 存活且 `-n-1` 为合法 settop 目标
  unsafe {
    lua_settop(l, -n - 1);
  }
}

/// 从 Lua 栈中弹出 `n` 个元素（对应 C/C++ 宏 `#define lua_pop(L,n) lua_settop(L, -(n)-1)`）。
///
/// # Safety
///
/// 调用方必须确保 `$l` 指向存活的 `LuaState`，且 `$n` 不能超过当前栈上的元素数。
#[macro_export]
macro_rules! lua_pop {
  ($l:expr, $n:expr) => {
    $crate::macros::lua_pop::lua_pop($l, ($n) as i32)
  };
}

pub use crate::lua_pop;
