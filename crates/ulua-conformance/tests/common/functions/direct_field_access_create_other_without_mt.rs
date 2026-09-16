use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::lua_newuserdatatagged::lua_newuserdatatagged, type_aliases::lua_state::lua_State,
};

use crate::common::{
  functions::direct_field_access_k_tag_other::K_TAG_OTHER,
  records::vec_2_direct_field_access_test::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_create_other_without_mt(
  l: *mut lua_State,
) -> c_int {
  unsafe {
    lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_OTHER);
    1
  }
}
