use core::ptr::null_mut;

use crate::{
  functions::{l_alloc::l_alloc, lua_newstate::lua_newstate},
  type_aliases::lua_state::lua_State,
};

pub fn lua_l_newstate() -> *mut lua_State {
  unsafe { lua_newstate(Some(l_alloc), null_mut()) }
}
