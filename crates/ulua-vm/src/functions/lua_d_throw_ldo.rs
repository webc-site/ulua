//! Source: `VM/src/ldo.cpp` (ldo.cpp:162-165, hand-ported; C++-exceptions build flavor,
//! matching the catch_unwind-based lua_d_rawrunprotected)

use std::panic::panic_any;

use crate::records::{lua_exception::lua_exception, lua_state::LuaState};

/// 抛出 `lua_exception` panic 载荷（cpp throw 的 panic 镜像）。本函数体仅做
/// 构造与 `panic_any`，不解引用 `l`；`l` 的有效性前提由载荷的消费方（捕获路径
/// 的 `what()` 等）在其解引用点保证。
pub fn lua_d_throw(l: *mut LuaState, errcode: i32) -> ! {
  panic_any(lua_exception::new(l, errcode));
}
