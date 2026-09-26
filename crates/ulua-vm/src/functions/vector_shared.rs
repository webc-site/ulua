//! vector 库共享核心：栈向量的分量读写窗口，以及 abs/floor/ceil/sign 与 max/min 的逐分量骨架坍缩。
//!
//! `LUA_VECTOR_SIZE` 为编译期常量，四分量分支按配置消歧，行为与 C++ 逐函数展开等价。

use crate::{
  functions::{
    lua_gettop::lua_gettop,
    lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    luaui_signf::luaui_signf,
  },
  macros::{lua_lib_arm::lua_lib_arm, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// 分量读取窗口：把栈向量的 3/4 个分量一次取成定长数组。3 分量配置下第 4 位恒 `+0.0`
/// 且**不触碰**该槽内存（栈上一个 vector 只占 `LUA_VECTOR_SIZE` 个 TValue）。
///
/// # Safety
/// `v` 须指向 `LUA_VECTOR_SIZE` 个连续可读 `f32`（即 `luaL_checkvector` 的返回值）。
#[inline]
pub(crate) unsafe fn vector_components(v: *const f32) -> [f32; 4] {
  unsafe {
    [
      *v,
      *v.add(1),
      *v.add(2),
      if LUA_VECTOR_SIZE == 4 { *v.add(3) } else { 0.0 },
    ]
  }
}

/// 分量写回窗口：4 分量配置压全部 4 位，3 分量配置压前 3 位（三参重载自身把 w 补成 `0.0`，
/// 与 cpp `lua_pushvector` 的两个重载逐位同形）。
///
/// # Safety
/// `l` 须为可正常压栈的存活 `LuaState`（扩容由 `lua_pushvector*` 内部 `ensure_stack` 完成）。
#[inline]
pub(crate) unsafe fn vector_push(l: *mut LuaState, c: [f32; 4]) {
  unsafe {
    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(l, c[0], c[1], c[2], c[3]);
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l, c[0], c[1], c[2]);
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
///
/// # Safety
/// `l` 须为受保护帧内的存活 `LuaState`：索引 1 须为 vector（否则 `luaL_checkvector` 抛错），
/// 返回指针覆盖 `LUA_VECTOR_SIZE` 个分量（读 [0..=3]）；压栈需 `(*l).top` 后 ≥1 空槽；可触发 GC。
pub(crate) unsafe fn vector_map1(l: *mut LuaState, f: impl Fn(f32) -> f32) -> i32 {
  // Safety: 契约保证索引 1 为 vector，其分量窗口与压栈均在 `l` 的帧界内
  unsafe {
    let v = vector_components(lua_l_checkvector(l, 1));
    vector_push(l, [f(v[0]), f(v[1]), f(v[2]), f(v[3])]);
    1
  }
}

/// 逐分量极值折叠核心：以首个实参为初值，对 2..=n 号实参按 `replace(候选, 当前)` 逐分量替换后压回。
///
/// # Safety
/// `l` 须为受保护帧内的存活 `LuaState`：实参数 n≥1，索引 1..=n 逐个 `luaL_checkvector`
/// （任一非 vector 即抛错，返回指针覆盖 `LUA_VECTOR_SIZE` 个分量）；压栈需 `(*l).top` 后 ≥1 空槽；可触发 GC。
pub(crate) unsafe fn vector_minmax(l: *mut LuaState, replace: impl Fn(f32, f32) -> bool) -> i32 {
  // Safety: 契约保证各索引为 vector，分量窗口与末尾压栈均在 `l` 的帧界内
  unsafe {
    let n = lua_gettop(l);
    let mut result = vector_components(lua_l_checkvector(l, 1));

    for i in 2..=n {
      // 候选分量按配置截取（size==3 时第 4 位恒 0.0，与 cpp 不读 b[3] 等值：
      // 该 lane 的 replace 判定在 0.0 上从不成立，也从不被压回）
      let cand = vector_components(lua_l_checkvector(l, i));

      for (r, x) in result.iter_mut().zip(cand) {
        if replace(x, *r) {
          *r = x;
        }
      }
    }

    vector_push(l, result);
    1
  }
}

lua_lib_arm! {
  /// # Safety
  /// 同上；vector 臂契约详见 [vector_map1]/[vector_minmax]。
  pub(crate) fn vector_abs(l) { unsafe { vector_map1(l, f32::abs) } }
}

lua_lib_arm! {
  /// # Safety
  /// 同上；vector 臂契约详见 [vector_map1]/[vector_minmax]。
  pub(crate) fn vector_ceil(l) { unsafe { vector_map1(l, f32::ceil) } }
}

lua_lib_arm! {
  /// # Safety
  /// 同上；vector 臂契约详见 [vector_map1]/[vector_minmax]。
  pub(crate) fn vector_floor(l) { unsafe { vector_map1(l, f32::floor) } }
}

lua_lib_arm! {
  /// # Safety
  /// 同上；vector 臂契约详见 [vector_map1]/[vector_minmax]。
  pub(crate) fn vector_sign(l) { unsafe { vector_map1(l, luaui_signf) } }
}

lua_lib_arm! {
  /// # Safety
  /// 同上；vector 臂契约详见 [vector_map1]/[vector_minmax]。
  pub(crate) fn vector_max(l) { unsafe { vector_minmax(l, |x, cur| x > cur) } }
}

lua_lib_arm! {
  /// # Safety
  /// 同上；vector 臂契约详见 [vector_map1]/[vector_minmax]。
  pub(crate) fn vector_min(l) { unsafe { vector_minmax(l, |x, cur| x < cur) } }
}
