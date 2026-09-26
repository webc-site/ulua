use core::ptr::null_mut;

use crate::{
  functions::{l_alloc::l_alloc, lua_newstate::lua_newstate},
  records::lua_state::LuaState,
};

pub fn lua_l_newstate() -> *mut LuaState {
  // Safety: 以默认 `l_alloc` 为分配器创建独立状态，ud 传空指针合法（分配器契约接受 null）
  unsafe { lua_newstate(Some(l_alloc), null_mut()) }
}
