use crate::type_aliases::lua_state::LuaState;

pub type StateRef = (
  *mut LuaState,
  Option<unsafe extern "C-unwind" fn(*mut LuaState)>,
);
