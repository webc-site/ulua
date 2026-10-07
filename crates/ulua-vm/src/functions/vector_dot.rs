use crate::{
  functions::vector_shared::check_vector,
  macros::{lua_lib_fn::lua_lib_fn, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// 两实参向量的点积，结果以 number 压回。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调，`l` 的存活与独占由 `&mut` 接收者
/// 承载且处于受保护帧——索引 1、2 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），
/// 分量读窗为 `[f32; 4]` 值拷贝、不持有栈指针；`push_number` 自身扩栈，可触发 GC。
pub(crate) fn vector_dot(l: &mut LuaState) -> i32 {
  let a = check_vector(l, 1);
  let b = check_vector(l, 2);

  // 逐项乘积可为 -0.0，故 3 分量配置不并入第 4 项（`x + 0.0` 会把 -0.0 归一为 +0.0）
  let mut d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  if LUA_VECTOR_SIZE == 4 {
    d += a[3] * b[3];
  }

  l.push_number(d as f64);
  1
}

lua_lib_fn!(pub(crate) fn vector_dot @ref, vector_dot_arm);
