//! math 库共享核心：单目/二目浮点函数族骨架坍缩。
//!
//! abs/acos/asin/atan/ceil/cos/cosh/deg/exp/floor/rad/round/sign/sin/sinh/sqrt/tan/tanh
//! 均为「取 1 号数字实参 → 单目变换 → 压回结果」，统一走 [math_map1]；
//! atan2/fmod/pow 为「取 1/2 号数字实参（按序）→ 二目变换 → 压回结果」，统一走 [math_map2]；
//! isnan/isinf/isfinite 为「取 1 号数字实参 → 谓词判定 → 压回布尔」，统一走 [math_pred1]；
//! max/min 为「1 号实参为初值 → 2..=n 号逐个择优 → 压回结果」，统一走 [math_extreme]。

use crate::records::lua_state::LuaState;

/// 单目数学核心：校验栈索引 1 为数字，套 `f` 后压回，返回结果数 1。
///
/// 借用形态与 vector 库共享核心同款（review §2）：核心收 `&mut LuaState`，
/// 栈校验/写回全为 `LuaState` 安全方法，raw 指针→引用的边界转换留在各 C 臂。
pub(crate) fn math_map1(l: &mut LuaState, f: impl Fn(f64) -> f64) -> i32 {
  let v = f(l.check_number(1));
  l.push_number(v);
  1
}

/// 二目数学核心：按 1、2 号顺序校验数字，套 `f` 后压回，返回结果数 1
/// （atan2/fmod/pow 同形骨架，求值顺序与各自原展开的实参序一致）。
pub(crate) fn math_map2(l: &mut LuaState, f: impl Fn(f64, f64) -> f64) -> i32 {
  let a = l.check_number(1);
  let b = l.check_number(2);

  l.push_number(f(a, b));
  1
}

/// 单目谓词核心：校验栈索引 1 为数字，压回 `f` 判定布尔，返回结果数 1。
pub(crate) fn math_pred1(l: &mut LuaState, f: impl Fn(f64) -> bool) -> i32 {
  let v = f(l.check_number(1));
  l.push_boolean(v);
  1
}

/// n 元极值核心：以 1 号实参为初值，2..=n 号实参按 `better(候选, 当前)` 逐个替换，
/// 结果压回，返回结果数 1（max/min 同形骨架）。
pub(crate) fn math_extreme(l: &mut LuaState, better: impl Fn(f64, f64) -> bool) -> i32 {
  let n = l.get_top();
  let mut acc = l.check_number(1);

  for i in 2..=n {
    let d = l.check_number(i);
    if better(d, acc) {
      acc = d;
    }
  }

  l.push_number(acc);
  1
}
