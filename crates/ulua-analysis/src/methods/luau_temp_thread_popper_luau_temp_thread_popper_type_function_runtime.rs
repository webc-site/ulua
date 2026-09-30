use ulua_vm::records::lua_state::LuaState as VmLuaState;

use crate::{
  records::luau_temp_thread_popper::LuauTempThreadPopper, type_aliases::lua_state::LuaState,
};

impl LuauTempThreadPopper {
  pub fn new(l: *mut LuaState) -> Self {
    Self { l }
  }
}

impl LuauTempThreadPopper {}

impl LuauTempThreadPopper {
  pub fn luau_temp_thread_popper(&mut self) {
    unsafe {
      (*(self.l as *mut VmLuaState)).pop(1);
    }
  }
}
