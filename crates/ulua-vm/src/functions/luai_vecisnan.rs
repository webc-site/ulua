use core::slice;

use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;

/// # Safety
/// `a` must point to at least LUA_VECTOR_SIZE contiguous floats.
#[inline]
pub(crate) unsafe fn luai_vecisnan(a: *const f32) -> bool {
  // Safety: 契约保证 v 指向 4 个连续可读 f32，add(0..=3) 的分量检查不越过 payload 界
  unsafe {
    slice::from_raw_parts(a, LUA_VECTOR_SIZE as usize)
      .iter()
      .any(|v| v.is_nan())
  }
}
