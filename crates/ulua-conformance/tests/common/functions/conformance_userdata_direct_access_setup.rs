use ulua_vm::{
  functions::{
    lua_callbacks::lua_callbacks,
    lua_registeruserdatadirectaccess::lua_registeruserdatadirectaccess,
  },
  records::lua_state::lua_State,
};

use crate::common::functions::{
  conformance_userdata_direct_access_useratom::conformance_userdata_direct_access_useratom,
  lua_vec_2_push::K_TAG_VEC2, lua_vertex_push::K_TAG_VERTEX,
  setup_userdata_helpers::setup_userdata_helpers, setup_vector_helpers::setup_vector_helpers,
  vec_2_direct_index::vec_2_direct_index, vec_2_direct_namecall::vec_2_direct_namecall,
  vec_2_direct_newindex::vec_2_direct_newindex, vertex_direct_index::vertex_direct_index,
  vertex_direct_namecall::vertex_direct_namecall, vertex_direct_newindex::vertex_direct_newindex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_direct_access_setup(l: *mut lua_State) {
  unsafe {
    (*lua_callbacks(l)).useratom = Some(conformance_userdata_direct_access_useratom);

    setup_vector_helpers(l);
    setup_userdata_helpers(l);

    let vec2_ok = lua_registeruserdatadirectaccess(
      l,
      K_TAG_VEC2,
      Some(vec_2_direct_index),
      Some(vec_2_direct_newindex),
      Some(vec_2_direct_namecall),
    );
    assert_eq!(vec2_ok, 1);

    let vertex_ok = lua_registeruserdatadirectaccess(
      l,
      K_TAG_VERTEX as i32,
      Some(vertex_direct_index),
      Some(vertex_direct_newindex),
      Some(vertex_direct_namecall),
    );
    assert_eq!(vertex_ok, 1);
  }
}
