//! 栈索引归一（cpp `lua_absindex` 的纯算术部分）：在压弹发生之前把当时的栈索引
//! 折成绝对索引，之后栈顶如何位移都指向同一槽位。
//!
//! 伪索引（`LUA_REGISTRYINDEX` 及其下方的 upvalue/环境区）与栈顶无关，原样返回；
//! 本 crate 的调用方只可能传注册表伪索引或真实栈槽索引。

use core::ffi::c_int;

use ulua_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;

/// `idx` 为待归一的栈索引、`top` 为取该索引时的栈高（`lua_gettop`）。
pub(crate) const fn to_absolute(idx: c_int, top: c_int) -> c_int {
  if idx > 0 || idx <= LUA_REGISTRYINDEX {
    idx
  } else {
    top + idx + 1
  }
}
