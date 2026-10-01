use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_vm::records::lua_state::LuaState;

#[derive(Debug, Clone, Default)]
pub struct Coverage {
  /// cpp `Coverage::L`（可空）：未 init 即 `None`，null 哨兵用 Option 表达。
  pub(crate) l: Option<NonNull<LuaState>>,
  pub(crate) functions: Vec<i32>,
}
