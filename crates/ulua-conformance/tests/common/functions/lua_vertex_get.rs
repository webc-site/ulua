use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::safe_api::{l_typeerror, touserdatatagged},
  records::{userdata_tags::K_TAG_VERTEX, vertex::Vertex},
};

pub(crate) fn lua_vertex_get(l: *mut LuaState, idx: i32) -> *mut Vertex {
  let a = touserdatatagged(l, idx, K_TAG_VERTEX as c_int) as *mut Vertex;

  if !a.is_null() {
    return a;
  }

  // tag 不符按 cpp 抛「不是 vertex」的类型错误，该调用不返回。
  l_typeerror(l, idx, "vertex")
}
