use ulua_vm::{macros::lua_pop::lua_pop, records::lua_state::LuaState as VmLuaState};

use crate::records::luau_temp_thread_popper::LuauTempThreadPopper;

impl LuauTempThreadPopper {
  pub fn luau_temp_thread_popper(&mut self) {
    unsafe {
      lua_pop(self.l as *mut VmLuaState, 1);
    }
  }
}
