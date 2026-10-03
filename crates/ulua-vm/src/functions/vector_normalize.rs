use crate::{
  functions::vector_shared::{check_vector, sum_squares, vector_push},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 实参向量归一化（各分量乘平方和开方的倒数），结果压回。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：以 Lua 库函数
/// 约定处于受保护帧，索引 1 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），分量读窗
/// 为 `[f32; 4]` 值拷贝；归一化为纯算术、不设零向量守卫（与收形前逐位同形）。结果槽由
/// `vector_push` 内部扩栈，可触发 GC。
pub(crate) fn vector_normalize(l: &mut LuaState) -> i32 {
  let v = check_vector(l, 1);

  let inv_sqrt = 1.0f32 / sum_squares(v).sqrt();
  vector_push(l, v.map(|x| x * inv_sqrt));

  1
}

lua_lib_fn!(pub(crate) fn vector_normalize @ref, vector_normalize_arm);
