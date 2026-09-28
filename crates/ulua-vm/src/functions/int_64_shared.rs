//! int64 库共享核心：单目/双目运算、比较族、按位折叠族与 n 元极值族骨架坍缩。
//!
//! bnot/neg/countlz/countrz/bswap 均为「取 1 号整数实参 → 单目运算 → 压整数」，
//! add/sub/mul 均为「取 1/2 号整数实参 → 双目 wrapping 运算 → 压整数」，
//! lt/le/gt/ge/ult/ule/ugt/uge 均为「取 1/2 号整数实参 → 二目比较 → 压布尔」，
//! band/bor/bxor 均为「gettop 定界 → 1..=n 号实参按 u64 折叠 → 转 i64 压整数」，
//! 统一走 [int64_fold_push]（btest 折叠后压布尔，直接用 [int64_fold]），
//! max/min 均为「1 号实参为初值 → 2..=n 号逐个择优 → 压回结果」，
//! udiv/urem 均为「取 1/2 号整数按 u64 重解读 → 除零校验 → 二目无符号运算 → 压回」，
//! div/idiv/rem/mod 均为「取 1/2 号整数实参 → 除数校验 → 二目有符号运算 → 压回」，
//! lshift/rshift 均为「取 1/2 号整数实参 → 位移量 -63..=63 收口（越界压 0）→ 二目移位 → 压回」，
//! lrotate/rrotate 均为「取 1/2 号整数实参 → 旋转量按 u64 取模 64 → 二目旋转（0 保原值）→ 压回」，
//! 其余单目/双目/比较/折叠/极值族统一走
//! [int64_unop]/[int64_binop]/[int64_cmp]/[int64_fold]/[int64_extreme]/[int64_uarith]/[int64_divop]/[int64_shiftop]/[int64_rotateop]
//! （泛型闭包单态化内联，零额外开销）。
//!
//! 全族核心均为 safe `fn(&mut LuaState, …)`：读槽（`check_integer_64`）、定界
//! （`get_top`）与压栈（`push_integer_64`/`push_boolean`）皆走 `LuaState` 的
//! safe 方法族，`unsafe` 只留在 C 臂的 `l: *mut LuaState → &mut` 边界转换与
//! FFI 调用内部（review §2：纯逻辑层不得持有 unsafe）。

use crate::{functions::check_div_args_64::check_nonzero_divisor, records::lua_state::LuaState};

/// 移位量绝对值上界（cpp `VM/src/lintlib.cpp:374/387/400` `if ((i >= -63) && (i <= 63))`）：
/// 区间外 lshift/rshift 一律压 0（arshift 另按符号补 0/-1），同时构成对 Rust
/// `<<`/`>>` 越界 shift 的防御性收口。
pub(crate) const INT64_SHIFT_ABS_MAX: i64 = u64::BITS as i64 - 1;

/// 旋转量归一模数：与 [`INT64_SHIFT_ABS_MAX`] 同为 u64 位宽导出量
/// （cpp `VM/src/lintlib.cpp:413/423` `(uint64_t)luaL_checkinteger64(L, 2) % 64`），
/// 由类型宽度编译期推导而非字面量。
const INT64_ROT_MOD: u64 = u64::BITS as u64;

/// 单目整数核心：校验栈索引 1 为整数，按 `f` 计算后压整数结果（bnot/neg/countlz/countrz/bswap 同形骨架）。
#[inline]
pub(crate) fn int64_unop(l: &mut LuaState, f: impl Fn(i64) -> i64) -> i32 {
  let result = f(l.check_integer_64(1));
  l.push_integer_64(result);

  1
}

/// 双目整数核心：校验栈索引 1、2 为整数（顺序读槽），按 `f` 计算后压整数结果（add/sub/mul 同形骨架）。
#[inline]
pub(crate) fn int64_binop(l: &mut LuaState, f: impl Fn(i64, i64) -> i64) -> i32 {
  let a = l.check_integer_64(1);
  let b = l.check_integer_64(2);

  l.push_integer_64(f(a, b));

  1
}

/// 二目比较核心：校验栈索引 1、2 为整数，按 `f` 比较后压布尔。
#[inline]
pub(crate) fn int64_cmp(l: &mut LuaState, f: impl Fn(i64, i64) -> bool) -> i32 {
  let a = l.check_integer_64(1);
  let b = l.check_integer_64(2);

  l.push_boolean(f(a, b));

  1
}

/// 按位折叠核心：`init` 起从索引 1..=n 逐个取整数实参按 u64 用 `f` 折叠，返回累加值（不压栈）。
#[inline]
pub(crate) fn int64_fold(l: &mut LuaState, init: u64, f: impl Fn(u64, u64) -> u64) -> u64 {
  let mut acc = init;

  for i in 1..=l.get_top() {
    acc = f(acc, l.check_integer_64(i) as u64);
  }

  acc
}

/// 折叠压整数核心：[int64_fold] 折叠 n 元实参后转 i64 压整数，返回结果数 1
/// （band/bor/bxor 同形骨架）。
#[inline]
pub(crate) fn int64_fold_push(l: &mut LuaState, init: u64, f: impl Fn(u64, u64) -> u64) -> i32 {
  let acc = int64_fold(l, init, f);
  l.push_integer_64(acc as i64);
  1
}

/// n 元极值核心：以 1 号实参为初值，2..=n 号实参按 `better(候选, 当前)` 逐个替换，
/// 结果压回，返回结果数 1（max/min 同形骨架）。
#[inline]
pub(crate) fn int64_extreme(l: &mut LuaState, better: impl Fn(i64, i64) -> bool) -> i32 {
  let mut acc = l.check_integer_64(1);

  for i in 2..=l.get_top() {
    let x = l.check_integer_64(i);
    if better(x, acc) {
      acc = x;
    }
  }

  l.push_integer_64(acc);
  1
}

/// 有符号除法族核心（div/idiv/rem/mod 同形骨架）：取 1/2 号整数实参，
/// 先经 `check(l, a, b)` 做除数校验（除零 / `i64::MIN ÷ -1` 单点抛错回退），
/// 再按 `f(a, b)` 计算压整数。div/idiv 传入 `check_div_args_64` 的闭包壳，
/// rem/mod 只查除零（各自在 `f` 内特判 `i64::MIN % -1`）。`check` 以泛型单态化。
#[inline]
pub(crate) fn int64_divop(
  l: &mut LuaState,
  check: impl Fn(&mut LuaState, i64, i64),
  f: impl Fn(i64, i64) -> i64,
) -> i32 {
  let a = l.check_integer_64(1);
  let b = l.check_integer_64(2);

  check(l, a, b);

  l.push_integer_64(f(a, b));

  1
}

/// 无符号二目算术核心（udiv/urem 同形骨架）：取 1/2 号整数实参按 u64 重解读，
/// 除数为 0 经 [`check_nonzero_divisor`] 单点抛错，`f(a, b)` 结果按 i64 压回。
#[inline]
pub(crate) fn int64_uarith(l: &mut LuaState, f: impl Fn(u64, u64) -> u64) -> i32 {
  let a = l.check_integer_64(1) as u64;
  let b = l.check_integer_64(2) as u64;

  check_nonzero_divisor(l, b);

  l.push_integer_64(f(a, b) as i64);

  1
}

/// 移位族核心（lshift/rshift 同形骨架）：取 1/2 号整数实参，位移量落在 `-63..=63`
/// 内时按 `f(n, i)` 计算后压整数，越界压 0（区间收口兼防 `<<`/`>>` 越界 shift）。
#[inline]
pub(crate) fn int64_shiftop(l: &mut LuaState, f: impl Fn(u64, i64) -> u64) -> i32 {
  let n = l.check_integer_64(1) as u64;
  let i = l.check_integer_64(2);

  let result = if (-INT64_SHIFT_ABS_MAX..=INT64_SHIFT_ABS_MAX).contains(&i) {
    f(n, i)
  } else {
    0
  };

  l.push_integer_64(result as i64);

  1
}

/// 旋转族核心（lrotate/rrotate 同形骨架）：取 1/2 号整数实参，旋转量按 u64 重解读后
/// `% INT64_ROT_MOD` 归一（负量经 u64 回绕，与 cpp 的 `uint64_t` 取模一致，
/// `VM/src/lintlib.cpp:413/423`），非 0 时按 `f(n, s)` 旋转压回，0 时原值压回。
#[inline]
pub(crate) fn int64_rotateop(l: &mut LuaState, f: impl Fn(u64, u32) -> u64) -> i32 {
  let n = l.check_integer_64(1) as u64;
  let s = (l.check_integer_64(2) as u64 % INT64_ROT_MOD) as u32;

  let result = if s != 0 { f(n, s) } else { n };

  l.push_integer_64(result as i64);

  1
}
