use crate::records::{global_state::global_State, lua_state::lua_State};
#[derive(Debug)]
#[repr(C)]
pub struct lg {
  pub l: lua_State,
  pub g: global_State,
}

pub type LG = lg;
