use core::sync::atomic::Ordering;

use ulua_vm::{
  functions::{lua_break::lua_break, lua_isyieldable::lua_isyieldable},
  records::lua_state::lua_State,
};

use crate::common::{
  functions::get_first_luau_frame_debug_info::get_first_luau_frame_debug_info,
  records::conformance_tag_method_error_state::CONFORMANCE_TAG_METHOD_ERROR_STATE,
};

const EXPECTED_HITS: [i32; 3] = [37, 54, 73];
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_debug_protected_error(
  l: *mut lua_State,
) {
  unsafe {
    let ar = get_first_luau_frame_debug_info(l);

    assert_ne!(lua_isyieldable(l), 0);
    let ar = ar.expect("expected a Lua frame");

    let index = CONFORMANCE_TAG_METHOD_ERROR_STATE
      .index
      .fetch_add(1, Ordering::SeqCst);
    assert!((index as usize) < EXPECTED_HITS.len());
    assert_eq!(ar.currentline, EXPECTED_HITS[index as usize]);

    if CONFORMANCE_TAG_METHOD_ERROR_STATE
      .lua_break
      .load(Ordering::SeqCst)
    {
      lua_break(l);
    }
  }
}
