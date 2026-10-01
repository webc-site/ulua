use core::{ffi::c_int, mem::size_of};

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::safe_api::{getuserdatametatable, newuserdatatagged, state_mut},
  records::{userdata_tags::K_TAG_VERTEX, vertex::Vertex},
};

pub(crate) fn lua_vertex_push(l: *mut LuaState) -> *mut Vertex {
  let data = newuserdatatagged(l, size_of::<Vertex>(), K_TAG_VERTEX as c_int) as *mut Vertex;

  getuserdatametatable(l, K_TAG_VERTEX as c_int);
  state_mut(l).set_metatable(-2);

  data
}
