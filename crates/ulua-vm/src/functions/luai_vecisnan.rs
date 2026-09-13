/// # Safety
/// `a` must point to at least 3 (or 4, depending on LUA_VECTOR_SIZE) contiguous floats.
use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;
#[inline]
pub(crate) unsafe fn luai_vecisnan(a: *const f32) -> bool {
  if LUA_VECTOR_SIZE == 4 {
    unsafe {
      let v0 = *a;
      let v1 = *a.add(1);
      let v2 = *a.add(2);
      let v3 = *a.add(3);
      v0.is_nan() || v1.is_nan() || v2.is_nan() || v3.is_nan()
    }
  } else {
    unsafe {
      let v0 = *a;
      let v1 = *a.add(1);
      let v2 = *a.add(2);
      v0.is_nan() || v1.is_nan() || v2.is_nan()
    }
  }
}
