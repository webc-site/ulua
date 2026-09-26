use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;

/// # Safety
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
#[inline]
pub(crate) unsafe fn luai_veceq(a: *const f32, b: *const f32) -> bool {
  // Safety: 契约保证 a/b 各指向 4 个连续可读 f32，add(0..=3) 的分量比较不越过 payload 界
  unsafe {
    if LUA_VECTOR_SIZE == 4 {
      *a == *b && *a.add(1) == *b.add(1) && *a.add(2) == *b.add(2) && *a.add(3) == *b.add(3)
    } else {
      *a == *b && *a.add(1) == *b.add(1) && *a.add(2) == *b.add(2)
    }
  }
}
