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
/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：本帧把该裸指针重建为独占引用（借用窗覆盖整个函数体），
/// 其后按 Lua 库函数约定——索引 1/2/3 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），
/// min/max 的 x/y/z 三分量关系逐个 `arg_check`（不满足即抛错回退），结果槽由 `vector_push` 内部
/// 扩栈，可触发 GC。
///
/// 签名保留裸指针形是 C ABI 透传壳 `ulua_vector_clamp`（ulua-capi，本票范围外）直呼本核心所致，
/// 与 `@ref` 族的差异仅在首参；壳侧改走 `_arm` 后即可与前移后的同族齐形。
pub unsafe fn vector_clamp(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为本次受保护帧内存活且无别名的 `LuaState`；此处仅做 C 臂
  // `l: *mut LuaState → &mut` 的边界转换，分量读窗与压栈皆走 safe 门面。
  let l = unsafe { &mut *l };

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

lua_lib_fn!(pub fn vector_clamp, vector_clamp_arm);
