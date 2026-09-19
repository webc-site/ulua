use ulua_vm::{
  functions::{lua_l_typeerror_l::lua_l_typeerror_l, lua_touserdatatagged::lua_touserdatatagged},
  records::lua_state::lua_State,
};

use crate::common::records::vertex::Vertex;
const K_TAG_VERTEX: i32 = 13;

pub(crate) fn lua_vertex_get(l: *mut lua_State, idx: i32) -> *mut Vertex {
  unsafe {
    let a = lua_touserdatatagged(l, idx, K_TAG_VERTEX) as *mut Vertex;

    if !a.is_null() {
      return a;
    }

    lua_l_typeerror_l(l, idx, "vertex");
  }
}
