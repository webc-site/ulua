use core::array::from_fn;

use crate::{
  functions::{
    luai_lerpf::luai_lerpf,
    vector_shared::{check_vector, vector_push},
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 两实参向量按第 3 号数值参数逐分量线性插值，结果压回。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调，`l` 的存活与独占由 `&mut`
/// 接收者承载且处于受保护帧——索引 1、2 须为 vector、索引 3 须为数值（否则
/// `check_vector`/`check_number` 抛错发散），分量读窗为 `[f32; 4]` 值拷贝；结果槽由
/// `vector_push` 内部扩栈，可触发 GC。
pub(crate) fn vector_lerp(l: &mut LuaState) -> i32 {
  let a = check_vector(l, 1);
  let b = check_vector(l, 2);
  let t = l.check_number(3) as f32;

  // luai_lerpf 为纯算术：3 分量配置下第 4 位（两端皆 0.0）算出即弃
  vector_push(l, from_fn(|i| luai_lerpf(a[i], b[i], t)));

  1
}

lua_lib_fn!(pub(crate) fn vector_lerp @ref, vector_lerp_arm);
