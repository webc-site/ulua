use crate::{
  functions::vector_shared::{check_vector, opt_vector},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// `vector.angle`：两向量夹角（弧度，`atan2(|a×b|, a·b)`），可选第 3 实参为旋转轴，
/// 叉积与该轴点积为负时取反角。
///
/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；按 Lua 库函数
/// 约定——索引 1、2 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），索引 3 可选
/// vector（缺参/nil 记为无轴，给出但非 vector 同样抛错发散）；只取 x/y/z 三个分量（w 不参与，
/// 与 cpp 的 3 分量读窗等值）。`push_number` 自身扩栈，可触发 GC；裸指针边界的内存契约见
/// `lua_lib_fn!` 单源生成的 `vector_angle_arm` `# Safety`。
pub fn vector_angle(l: &mut LuaState) -> i32 {
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

lua_lib_fn!(pub fn vector_angle @ref, vector_angle_arm);
