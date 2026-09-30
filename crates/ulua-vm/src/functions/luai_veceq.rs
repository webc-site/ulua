use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;

/// 向量分量逐一相等判定（cpp `luai_veceq`）。操作数是定长 `[f32; LUA_VECTOR_SIZE]`
/// 视图，切片相等的逐元素比较与原 `*a.add(i)` 手抄分量比较逐位等价，故无需裸指针。
#[inline]
pub(crate) fn luai_veceq(
  a: &[f32; LUA_VECTOR_SIZE as usize],
  b: &[f32; LUA_VECTOR_SIZE as usize],
) -> bool {
  a == b
}
