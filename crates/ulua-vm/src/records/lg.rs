use crate::records::{global_state::global_State, lua_state::LuaState};
#[derive(Debug)]
#[repr(C)]
pub struct lg {
  pub l: LuaState,
  pub g: global_State,
}

pub type LG = lg;
