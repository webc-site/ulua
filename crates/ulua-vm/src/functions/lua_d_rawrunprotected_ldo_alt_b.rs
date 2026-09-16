//! Node: `cxx:Function:Luau.VM:VM/src/ldo.cpp:124:lua_d_rawrunprotected`（变体别名）
//!
//! 历史上这里有一份与 `lua_d_rawrunprotected_ldo` 平行的独立 `catch_unwind`
//! 实现（早期移植产物）。C++ 上游只有唯一一个 `luaD_rawrunprotected`，两条
//! 路径的 payload 分发、panic-hook 安装与 `catch (std::exception&)` 回退完全
//! 同构，故收敛为对 `luaD_rawrunprotected` 的纯委托；保留函数与导出符号，
//! `lua_d_pcall` / `shrinkstackprotected` 的调用点无需变动。

use core::ffi::c_void;

use crate::{
  functions::lua_d_rawrunprotected_ldo::luaD_rawrunprotected,
  type_aliases::{lua_state::lua_State, pfunc::Pfunc},
};

#[cfg_attr(
  feature = "capi",
  unsafe(export_name = "ulua_lua_d_rawrunprotected_mut")
)]
pub(crate) unsafe fn lua_d_rawrunprotected_mut(
  l: *mut lua_State,
  f: Pfunc,
  ud: *mut c_void,
) -> i32 {
  unsafe { luaD_rawrunprotected(l, f, ud) }
}
