use crate::macros::lua_vector_size::LUA_VECTOR_SIZE;

/// 向量分量是否存在 NaN（cpp `luai_vecisnan`）。操作数是定长
/// `[f32; LUA_VECTOR_SIZE]` 视图，与同族 `luai_veceq` 统形：分量读取即切片迭代，
/// 无需裸指针，故本函数为 safe（原 `*const f32` + 契约的形态只剩一次
/// 引用→指针→引用的往返）。
#[inline]
pub(crate) fn luai_vecisnan(a: &[f32; LUA_VECTOR_SIZE as usize]) -> bool {
  a.iter().any(|v| v.is_nan())
}
