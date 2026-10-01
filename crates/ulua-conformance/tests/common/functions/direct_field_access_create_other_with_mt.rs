use core::{ffi::c_int, mem::size_of};

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::{direct_field_access_k_tag_other::K_TAG_OTHER, safe_api::newuserdatataggedwithmetatable},
  records::vec_2_direct_field_access_test::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_create_other_with_mt(
  l: *mut LuaState,
) -> c_int {
  newuserdatataggedwithmetatable(l, size_of::<Vec2>(), K_TAG_OTHER);
  1
}
