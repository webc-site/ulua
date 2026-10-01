use core::{ffi::c_int, mem::size_of};

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::{direct_field_access_k_tag_vec_2::K_TAG_VEC2, safe_api::{newuserdatatagged, state_mut}},
  records::vec_2_direct_field_access_test::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_create_vec_2(l: *mut LuaState) -> c_int {
  let x = state_mut(l).check_number(1);
  let y = state_mut(l).check_number(2);

  let p = newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2) as *mut Vec2;
  // Safety: `p` 为刚分配的 Vec2 userdata 数据区，尺寸/对齐由上一行实参配对保证。
  unsafe {
    (*p).x = x;
    (*p).y = y;
  }

  1
}
