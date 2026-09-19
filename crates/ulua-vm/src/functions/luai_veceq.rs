use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub(crate) unsafe fn luai_veceq(a: *const f32, b: *const f32) -> bool {
  unsafe {
    if LUA_VECTOR_SIZE == 4 {
      *a == *b && *a.add(1) == *b.add(1) && *a.add(2) == *b.add(2) && *a.add(3) == *b.add(3)
    } else {
      *a == *b && *a.add(1) == *b.add(1) && *a.add(2) == *b.add(2)
    }
  }
}
