use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_newuserdatatagged::lua_newuserdatatagged},
  type_aliases::lua_state::lua_State,
};

use crate::common::{
  functions::direct_field_access_k_tag_vec_2::K_TAG_VEC2,
  records::vec_2_direct_field_access_test::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_create_vec_2(l: *mut lua_State) -> c_int {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);

    let p = lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2) as *mut Vec2;
    (*p).x = x;
    (*p).y = y;

    1
  }
}
