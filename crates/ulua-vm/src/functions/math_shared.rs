//! math 库共享核心：单目/二目浮点函数族骨架坍缩。
//!
//! abs/acos/asin/atan/ceil/cos/cosh/deg/exp/floor/rad/round/sign/sin/sinh/sqrt/tan/tanh
//! 均为「取 1 号数字实参 → 单目变换 → 压回结果」，统一走 [math_map1]；
//! atan2/fmod/pow 为「取 1/2 号数字实参（按序）→ 二目变换 → 压回结果」，统一走 [math_map2]；
//! isnan/isinf/isfinite 为「取 1 号数字实参 → 谓词判定 → 压回布尔」，统一走 [math_pred1]；
//! max/min 为「1 号实参为初值 → 2..=n 号逐个择优 → 压回结果」，统一走 [math_extreme]。

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checknumber::lua_l_checknumber, lua_pushboolean::lua_pushboolean,
    lua_pushnumber::lua_pushnumber,
  },
  records::lua_state::LuaState,
};

/// 单目数学核心：校验栈索引 1 为数字，套 `f` 后压回，返回结果数 1。
///
/// # Safety
/// `l` 须为受保护帧内的存活 `LuaState`：索引 1 经 `lua_l_checknumber` 强制为数字（缺失/非数字抛错），
/// `lua_pushnumber` 写回需 `(*l).top` 后 ≥1 空槽；可触发 GC。
pub(crate) unsafe fn math_map1(l: *mut LuaState, f: impl Fn(f64) -> f64) -> i32 {
  unsafe {
    lua_pushnumber(l, f(lua_l_checknumber(l, 1)));
    1
  }
}

/// 二目数学核心：按 1、2 号顺序校验数字，套 `f` 后压回，返回结果数 1
/// （atan2/fmod/pow 同形骨架，求值顺序与各自原展开的实参序一致）。
///
/// # Safety
/// `l` 须为受保护帧内的存活 `LuaState`：索引 1、2 逐个经 `lua_l_checknumber` 强制为数字
/// （任一缺失/非数字即抛错回退），`lua_pushnumber` 写回需 `(*l).top` 后 ≥1 空槽；可触发 GC。
pub(crate) unsafe fn math_map2(l: *mut LuaState, f: impl Fn(f64, f64) -> f64) -> i32 {
  unsafe {
    let a = lua_l_checknumber(l, 1);
    let b = lua_l_checknumber(l, 2);

    lua_pushnumber(l, f(a, b));
    1
  }
}

/// 单目谓词核心：校验栈索引 1 为数字，压回 `f` 判定布尔，返回结果数 1。
///
/// # Safety
/// 同 [math_map1]（写回改走 `lua_pushboolean`）。
pub(crate) unsafe fn math_pred1(l: *mut LuaState, f: impl Fn(f64) -> bool) -> i32 {
  unsafe {
    lua_pushboolean(l, f(lua_l_checknumber(l, 1)) as i32);
    1
  }
}

/// n 元极值核心：以 1 号实参为初值，2..=n 号实参按 `better(候选, 当前)` 逐个替换，
/// 结果压回，返回结果数 1（max/min 同形骨架）。
///
/// # Safety
/// `l` 须为受保护帧内的存活 `LuaState`：`lua_gettop` 定界后 1..=n 号位逐个经
/// `lua_l_checknumber` 强制为数字（任一缺失/非数字即抛错回退），`lua_pushnumber`
/// 写回需 `(*l).top` 后 ≥1 空槽；可触发 GC。
pub(crate) unsafe fn math_extreme(l: *mut LuaState, better: impl Fn(f64, f64) -> bool) -> i32 {
  unsafe {
    let n = lua_gettop(l);
    let mut acc = lua_l_checknumber(l, 1);

    for i in 2..=n {
      let d = lua_l_checknumber(l, i);
      if better(d, acc) {
        acc = d;
      }
    }

    lua_pushnumber(l, acc);
    1
  }
}
