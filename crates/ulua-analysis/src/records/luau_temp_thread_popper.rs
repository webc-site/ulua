use crate::type_aliases::lua_state::LuaState;

#[derive(Debug, Clone)]
pub struct LuauTempThreadPopper {
  pub(crate) l: *mut LuaState,
}
