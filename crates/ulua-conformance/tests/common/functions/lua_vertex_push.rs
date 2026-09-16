use core::mem::size_of;

use ulua_vm::{
  functions::{
    lua_getuserdatametatable::lua_getuserdatametatable,
    lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable,
  },
  records::lua_state::lua_State,
};

use crate::common::records::vertex::Vertex;
pub const K_TAG_VERTEX: u8 = 13;

pub(crate) fn lua_vertex_push(l: *mut lua_State) -> *mut Vertex {
  unsafe {
    let data = lua_newuserdatatagged(l, size_of::<Vertex>(), K_TAG_VERTEX as i32) as *mut Vertex;

    lua_getuserdatametatable(l, K_TAG_VERTEX as i32);
    lua_setmetatable(l, -2);

    data
  }
}
