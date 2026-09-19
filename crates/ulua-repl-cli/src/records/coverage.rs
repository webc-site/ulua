use alloc::vec::Vec;

use ulua_vm::type_aliases::lua_state::lua_State;

#[derive(Debug, Clone, Default)]
pub struct Coverage {
  pub(crate) l: *mut lua_State,
  pub(crate) functions: Vec<i32>,
}
