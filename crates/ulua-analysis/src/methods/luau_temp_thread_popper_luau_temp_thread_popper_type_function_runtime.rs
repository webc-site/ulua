

use ulua_vm::records::lua_state::LuaState as VmLuaState;
use crate::{records::{arena_handle::alias, luau_temp_thread_popper::LuauTempThreadPopper}};
impl LuauTempThreadPopper {
  pub fn new(l: &mut LuaState) -> Self {
    Self { l }
  }
}

impl LuauTempThreadPopper {}

impl LuauTempThreadPopper {
  pub fn luau_temp_thread_popper(&mut self) {
    alias(self.l as *mut VmLuaState).pop(1);
  }
}
