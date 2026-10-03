use crate::{
  functions::vector_shared::{check_vector, vector_push},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 两实参向量的叉积，结果压回。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调，`l` 的存活与独占由 `&mut` 接收者
/// 承载且处于受保护帧——索引 1、2 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），
/// 分量读窗为 `[f32; 4]` 值拷贝、不持有栈指针；结果槽由 `vector_push` 内部扩栈，可触发 GC。
pub(crate) fn vector_cross(l: &mut LuaState) -> i32 {
  let a = check_vector(l, 1);
  let b = check_vector(l, 2);

  // 叉积只落在 x/y/z 上，w 恒 0.0（cpp 四分量重载同形）
  vector_push(
    l,
    [
      a[1] * b[2] - a[2] * b[1],
      a[2] * b[0] - a[0] * b[2],
      a[0] * b[1] - a[1] * b[0],
      0.0f32,
    ],
  );

  1
}

lua_lib_fn!(pub(crate) fn vector_cross @ref, vector_cross_arm);
