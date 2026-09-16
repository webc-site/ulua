use ulua_vm::records::lua_state::lua_State;

use crate::common::{functions::lua_vertex_push::lua_vertex_push, records::vertex::Vertex};

pub(crate) fn lua_vertex_clone(l: *mut lua_State, self_ptr: *mut Vertex) -> i32 {
  unsafe {
    let r_ptr = lua_vertex_push(l);
    *r_ptr = *self_ptr;
    1
  }
}
