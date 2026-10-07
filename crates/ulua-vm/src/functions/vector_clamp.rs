use core::{
  array::from_fn,
  cmp::Ordering::{Equal, Less},
};

use crate::{
  functions::{
    luaui_clampf::luaui_clampf,
    vector_shared::{check_vector, vector_push},
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// `vector.clamp`：把索引 1 向量逐分量夹到索引 2/3 两向量给出的 min/max 区间内。
///
/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；按 Lua 库函数
/// 约定——索引 1/2/3 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），min/max 的
/// x/y/z 三分量关系逐个 `arg_check`（不满足即抛错回退），结果槽由 `vector_push` 内部扩栈，
/// 可触发 GC；裸指针边界的内存契约见 `lua_lib_fn!` 单源生成的 `vector_clamp_arm` `# Safety`。
pub fn vector_clamp(l: &mut LuaState) -> i32 {
  let v = check_vector(l, 1);
  let min = check_vector(l, 2);
  let max = check_vector(l, 3);

  l.arg_check(
    matches!(min[0].partial_cmp(&max[0]), Some(Less | Equal)),
    3,
    "max.x must be greater than or equal to min.x",
  );
  l.arg_check(
    matches!(min[1].partial_cmp(&max[1]), Some(Less | Equal)),
    3,
    "max.y must be greater than or equal to min.y",
  );
  l.arg_check(
    matches!(min[2].partial_cmp(&max[2]), Some(Less | Equal)),
    3,
    "max.z must be greater than or equal to min.z",
  );

  // luaui_clampf 为纯算术：3 分量配置下第 4 位（三端皆 0.0）算出即弃
  vector_push(l, from_fn(|i| luaui_clampf(v[i], min[i], max[i])));

  1
}

lua_lib_fn!(pub fn vector_clamp @ref, vector_clamp_arm);
