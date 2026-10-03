

#[derive(Debug, Clone)]
use ulua_vm::records::lua_state::LuaState;
pub struct LuauTempThreadPopper {
  pub(crate) l: &mut LuaState,
}
