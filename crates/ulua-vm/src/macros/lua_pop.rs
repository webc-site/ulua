use crate::{functions::lua_settop::lua_settop, records::lua_state::LuaState};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState`；`n` 须为不超过当前栈元素数的出栈个数（cpp 宏同款
/// `lua_settop(l, -(n)-1)` 语义）。
#[inline]
pub unsafe fn lua_pop(l: *mut LuaState, n: i32) {
  // Safety: 契约保证 `l` 存活且 `-n-1` 为合法 settop 目标
  unsafe {
    lua_settop(l, -n - 1);
  }
}
