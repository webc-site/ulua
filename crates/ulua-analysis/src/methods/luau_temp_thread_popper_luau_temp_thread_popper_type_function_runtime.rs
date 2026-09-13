use crate::{
  records::luau_temp_thread_popper::LuauTempThreadPopper, type_aliases::lua_state::LuaState,
};

impl LuauTempThreadPopper {
  pub fn new(l: *mut LuaState) -> Self {
    Self { l }
  }
}

impl LuauTempThreadPopper {}
