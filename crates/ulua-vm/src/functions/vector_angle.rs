use crate::{
  functions::vector_shared::{check_vector, opt_vector},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// `vector.angle`：两向量夹角（弧度，`atan2(|a×b|, a·b)`），可选第 3 实参为旋转轴，
/// 叉积与该轴点积为负时取反角。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：本帧把该裸指针重建为独占引用（借用窗覆盖整个
/// 函数体），其后按 Lua 库函数约定——索引 1、2 须为 vector（否则 `check_vector` 经
/// `tag_error` 抛错发散），索引 3 可选 vector（缺参/nil 记为无轴，给出但非 vector 同样抛错
/// 发散）；只取 x/y/z 三个分量（w 不参与，与 cpp 的 3 分量读窗等值）。`push_number` 自身扩栈，
/// 可触发 GC。
///
/// 签名保留裸指针形是 C ABI 透传壳 `ulua_vector_angle`（ulua-capi，本票范围外）直呼本核心
/// 所致，与 `@ref` 族的差异仅在首参；壳侧改走 `_arm` 后即可与前移后的同族齐形。
pub unsafe fn vector_angle(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为本次受保护帧内存活且无别名的 `LuaState`；此处仅做 C 臂
  // `l: *mut LuaState → &mut` 的边界转换，分量读窗与压栈皆走 safe 门面。
  let l = unsafe { &mut *l };

  let a = check_vector(l, 1);
  // cpp: luaL_checkvector(L, 2) —— b 是必需参数：换成 opt_vector 会漏检缺参（折成 `None`
  // 后无值可算），落不进 cpp 侧「缺参/非 vector 即抛 Lua 错误」的语义
  let b = check_vector(l, 2);
  let axis = opt_vector(l, 3);

  let cross = [
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  ];

  let sin_a = ((cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]) as f64).sqrt();
  let cos_a = (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) as f64;
  let mut angle = sin_a.atan2(cos_a);

  if let Some(axis) = axis
    && cross[0] * axis[0] + cross[1] * axis[1] + cross[2] * axis[2] < 0.0
  {
    angle = -angle;
  }

  l.push_number(angle);
  1
}

lua_lib_fn!(pub fn vector_angle, vector_angle_arm);
