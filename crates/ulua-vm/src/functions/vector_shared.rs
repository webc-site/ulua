//! vector 库共享核心：栈向量实参的分量读窗门面，以及 abs/floor/ceil/sign 与 max/min 的逐分量骨架坍缩。
//!
//! `LUA_VECTOR_SIZE` 为编译期常量，四分量分支按配置消歧，行为与 C++ 逐函数展开等价。
//! 读窗（[check_vector]/[opt_vector]）与写回（[vector_push]）都是 safe 门面，形参一律收
//! `&mut LuaState`：裸指针只在 [vector_components] 一处收成 `[f32; 4]` 值窗、C-ABI 压栈镜像只在
//! [vector_push] 一处重取裸参，两处的 `unsafe` 皆有契约（review §2）。map1/minmax 核心与 vector
//! 库各消费方因此全程只见 `&mut LuaState` + `[f32; 4]`：本族的 `LuaState` 裸指针形只余
//! `lua_lib_arm!`/`lua_lib_fn!` 生成的 `extern "C-unwind"` 边界臂。`vector_angle`/`vector_clamp`
//! 两核已随 ulua-capi 透传壳退役为显式壳而一并前移，本族 `unsafe fn` 因此归零。

use core::{ptr::null, slice::from_raw_parts};

use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_l_optvector::lua_l_optvector,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    luaui_signf::luaui_signf,
  },
  macros::{lua_lib_arm::lua_lib_arm, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// 分量读取窗口：把栈向量的 3/4 个分量一次取成定长数组。3 分量配置下第 4 位恒
/// `+0.0` 且**不触碰**该槽内存（栈上一个 vector 只占 `LUA_VECTOR_SIZE` 个 TValue）。
///
/// 本函数是全族唯一的分量窗指针边界，仅由 [check_vector]/[opt_vector] 两门面调用，
/// 消费方一律走门面、不再经手裸指针。
///
/// # Safety
/// `v` 须指向 `LUA_VECTOR_SIZE` 个连续可读 `f32`（即 `luaL_checkvector` 的返回值）。
#[inline]
fn vector_components(v: *const f32) -> [f32; 4] {
  // SAFETY: 上述契约——切片只覆盖 `LUA_VECTOR_SIZE` 个分量；`get(3)` 在 3 分量
  // 构建下不读第 4 槽（与收口前 `if LUA_VECTOR_SIZE == 4` 门逐位同形）。
  let lanes = unsafe { from_raw_parts(v, LUA_VECTOR_SIZE as usize) };
  [
    lanes[0],
    lanes[1],
    lanes[2],
    lanes.get(3).copied().unwrap_or(0.0),
  ]
}

/// 必需栈实参向量的分量窗口（`luaL_checkvector` + [vector_components] 的合点门面）：
/// `l` 的存活与独占由 `&mut` 接收者承载，返回值为纯 `f32` 数组、不携带指针。
///
/// 调用序契约（正确性，非内存安全）：须以 Lua 库函数约定在受保护帧内调用，索引 `narg`
/// 非 vector 时经 `tag_error` 抛错发散（本函数即不返回）。
#[inline]
pub(crate) fn check_vector(l: &mut LuaState, narg: i32) -> [f32; 4] {
  vector_components(lua_l_checkvector(l, narg))
}

/// 可选栈实参向量的分量窗口（cpp `luaL_optvector(L, narg, NULL)` + [vector_components]
/// 的合点门面）：实参缺失或为 nil 返回 `None`（cpp 的 `NULL` 默认值折成 `Option`，
/// review §2「可空指针 → `Option`」），否则按 [check_vector] 同形取窗（非 vector 抛错发散）。
#[inline]
pub(crate) fn opt_vector(l: &mut LuaState, narg: i32) -> Option<[f32; 4]> {
  // SAFETY: `l.as_mut_ptr()` 由 `&mut` 借用重取的存活帧裸参（`LuaState::as_mut_ptr` 口径）；
  // `def` 传 `null()` 即 cpp `luaL_optvector(L, n, NULL)` 形——缺参/nil 原样返回该 null，
  // 本门面据此短路、不构造窗口，故 null 从不流入 [vector_components]。
  let v = unsafe { lua_l_optvector(l.as_mut_ptr(), narg, null()) };
  if v.is_null() {
    None
  } else {
    Some(vector_components(v))
  }
}

/// 分量写回窗口门面：4 分量配置压全部 4 位，3 分量配置压前 3 位（三参重载自身把 w 补成
/// `0.0`，与 cpp `lua_pushvector` 的两个重载逐位同形）。扩容由 `lua_pushvector*`
/// 内部 `ensure_stack` 完成。
///
/// 调用序契约（正确性，非内存安全）：`l` 的存活与独占由 `&mut` 接收者承载；须处于可正常
/// 压栈的受保护帧，返回后此前缓存的栈/槽指针失效（`ensure_stack` 可能 realloc）。
#[inline]
pub(crate) fn vector_push(l: &mut LuaState, c: [f32; 4]) {
  // SAFETY: 本块只把借用重建的存活帧裸参转交给两个 `lua_pushvector*` C-ABI 镜像边界
  // （各自 `ensure_stack` 扩栈后写槽）。`LUA_VECTOR_SIZE == 4` 为编译期常量门，每次调用恰选
  // 一个重载：3 分量配置走三参重载，第 4 lane 无槽位、`c[3]` 亦不传给被调方，与收形前同形。
  unsafe {
    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(l.as_mut_ptr(), c[0], c[1], c[2], c[3]);
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l.as_mut_ptr(), c[0], c[1], c[2]);
    }
  }
}

/// 分量平方和（cpp 左结合逐项累加，与 `v0*v0 + v1*v1 + v2*v2 + v3*v3` 同序）。
///
/// 3 分量配置下尾项为 `0.0 * 0.0 = +0.0`：平方和恒非负，故 `x + 0.0` 不改变任何结果
/// （含 ±0.0、inf、NaN），与只加三项等值。
#[inline]
pub(crate) const fn sum_squares(c: [f32; 4]) -> f32 {
  c[0] * c[0] + c[1] * c[1] + c[2] * c[2] + c[3] * c[3]
}

/// 单目分量数学核心：校验栈索引 1 为 vector，各分量套 `f` 后压回，返回结果数 1。
///
/// `f` 为纯数值函数（abs/floor/ceil/sign），3 分量配置下第 4 位的 `f(0.0)` 计算后丢弃，
/// 无副作用、不改变压栈值。
pub(crate) fn vector_map1(l: &mut LuaState, f: impl Fn(f32) -> f32) -> i32 {
  let v = check_vector(l, 1);
  vector_push(l, [f(v[0]), f(v[1]), f(v[2]), f(v[3])]);
  1
}

/// 逐分量极值折叠核心：以首个实参为初值，对 2..=n 号实参按 `replace(候选, 当前)` 逐分量替换后压回。
pub(crate) fn vector_minmax(l: &mut LuaState, replace: impl Fn(f32, f32) -> bool) -> i32 {
  let n = l.get_top();
  let mut result = check_vector(l, 1);

  for i in 2..=n {
    // 候选分量按配置截取（size==3 时第 4 位恒 0.0，与 cpp 不读 b[3] 等值：
    // 该 lane 的 replace 判定在 0.0 上从不成立，也从不被压回）
    let cand = check_vector(l, i);

    for (r, x) in result.iter_mut().zip(cand) {
      if replace(x, *r) {
        *r = x;
      }
    }
  }

  vector_push(l, result);
  1
}

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `LuaState`，索引 1 为 vector（契约见 [vector_map1]）。
  pub(crate) fn vector_abs(l) { vector_map1(unsafe { &mut *l }, f32::abs) }
}

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `LuaState`，索引 1 为 vector（契约见 [vector_map1]）。
  pub(crate) fn vector_ceil(l) { vector_map1(unsafe { &mut *l }, f32::ceil) }
}

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `LuaState`，索引 1 为 vector（契约见 [vector_map1]）。
  pub(crate) fn vector_floor(l) { vector_map1(unsafe { &mut *l }, f32::floor) }
}

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `LuaState`，索引 1 为 vector（契约见 [vector_map1]）。
  pub(crate) fn vector_sign(l) { vector_map1(unsafe { &mut *l }, luaui_signf) }
}

lua_lib_arm! {
  /// # Safety
  /// 实参数 n≥1，索引 1..=n 逐个 `luaL_checkvector`（契约见 [vector_minmax]）。
  pub(crate) fn vector_max(l) { vector_minmax(unsafe { &mut *l }, |x, cur| x > cur) }
}

lua_lib_arm! {
  /// # Safety
  /// 实参数 n≥1，索引 1..=n 逐个 `luaL_checkvector`（契约见 [vector_minmax]）。
  pub(crate) fn vector_min(l) { vector_minmax(unsafe { &mut *l }, |x, cur| x < cur) }
}
