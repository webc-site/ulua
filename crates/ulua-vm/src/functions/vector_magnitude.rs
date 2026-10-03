use crate::{
  functions::vector_shared::{check_vector, sum_squares},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 实参向量的模（分量平方和开方），结果以 number 压回。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：以 Lua 库函数
/// 约定处于受保护帧，索引 1 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），
/// 分量读窗为 `[f32; 4]` 值拷贝、不持有栈指针；`push_number` 自身扩栈，可触发 GC。
pub(crate) fn vector_magnitude(l: &mut LuaState) -> i32 {
  let v = check_vector(l, 1);

  l.push_number(sum_squares(v).sqrt() as f64);
  1
}

lua_lib_fn!(pub(crate) fn vector_magnitude @ref, vector_magnitude_arm);
