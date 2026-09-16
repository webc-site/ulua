use ulua_vm::{functions::lua_l_checknumber::lua_l_checknumber, records::lua_state::lua_State};

use crate::common::functions::lua_vec_2_push::lua_vec_2_push;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2(l: *mut lua_State) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);

    let data = lua_vec_2_push(l);

    (*data).x = x as f32;
    (*data).y = y as f32;
  }
  1
}
