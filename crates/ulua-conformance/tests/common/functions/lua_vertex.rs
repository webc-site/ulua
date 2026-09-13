use ulua_vm::{functions::lua_l_checkvector::lua_l_checkvector, records::lua_state::lua_State};

use crate::common::functions::{lua_vec_2_get::lua_vec_2_get, lua_vertex_push::lua_vertex_push};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex(l: *mut lua_State) -> i32 {
  unsafe {
    let pos = lua_l_checkvector(l, 1);
    let normal = lua_l_checkvector(l, 2);
    let uv = lua_vec_2_get(l, 3);

    let data = lua_vertex_push(l);

    (*data).pos[0] = *pos;
    (*data).pos[1] = *pos.add(1);
    (*data).pos[2] = *pos.add(2);

    (*data).normal[0] = *normal;
    (*data).normal[1] = *normal.add(1);
    (*data).normal[2] = *normal.add(2);

    (*data).uv[0] = (*uv).x;
    (*data).uv[1] = (*uv).y;

    1
  }
}
