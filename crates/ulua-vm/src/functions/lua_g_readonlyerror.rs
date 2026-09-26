//! Source: `VM/src/ldebug.cpp:313-316` (hand-ported)

use crate::{
  macros::lua_g_runerror::lua_g_runerror,
  records::{lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可抛错的受保护帧；`lua_g_runerror!` 以固定文本组装错误对象并沿保护帧 unwind，
/// 本函数返回 `!`（永不正常返回）。cpp/VM/src/ldebug.cpp:346 luaG_readonlyerror。
pub unsafe fn lua_g_readonlyerror(l: *mut LuaState) -> ! {
  unsafe { lua_g_runerror!(l, "attempt to modify a readonly table") }
}

/// 只读表写守卫：`t.readonly` 非零即抛 [`lua_g_readonlyerror`]。
/// 收敛 cpp 中 lapi/lvmutils/ltablib 等处 11 个同形三行检查 `if (t->readonly) luaG_readonlyerror(L);`。
///
/// # Safety
/// 同 [`lua_g_readonlyerror`]：`l` 须为存活 state 且处于可抛错的受保护帧；`t` 须指向存活 `LuaTable`。
#[inline]
pub unsafe fn check_writable(l: *mut LuaState, t: *mut LuaTable) {
  // Safety: 契约保证 `t` 指向存活表且 `l` 处于可抛错受保护帧，读 readonly 标志与抛错均在该帧内合法
  unsafe {
    if (*t).readonly != 0 {
      lua_g_readonlyerror(l);
    }
  }
}
