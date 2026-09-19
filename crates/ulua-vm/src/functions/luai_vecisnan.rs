use core::slice;

use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;

/// # Safety
/// `a` must point to at least LUA_VECTOR_SIZE contiguous floats.
#[inline]
pub(crate) unsafe fn luai_vecisnan(a: *const f32) -> bool {
  unsafe {
    slice::from_raw_parts(a, LUA_VECTOR_SIZE as usize)
      .iter()
      .any(|v| v.is_nan())
  }
}
