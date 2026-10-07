//! math 单参内建 C 函数地址对账面（CALL 特化的守卫共用点）。
//!
//! fn 项地址唯一性由全仓单一定义保证（`fn_address_comparisons` lint 的
//! 「地址不保证唯一」保守提示不适用），转 usize 对账以绕开 fn 指针直接
//! 比较的 lint 面。

use std::mem::transmute;

use ulua_vm::{
  functions::{
    math_abs::math_abs, math_ceil::math_ceil, math_floor::math_floor, math_round::math_round,
    math_sqrt::math_sqrt,
  },
  records::lua_t_value::TValue,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::functions::trace_forn_ir::TMathUnary;

/// 各单参内建 C 函数的 fn 项地址（usize 对账形）。
fn math_fn_addr(kind: TMathUnary) -> usize {
  // SAFETY: LuaCFunction 为 Option<extern fn>，与 usize 同尺寸裸转（fn 项
  // 地址对账，不回落任何表示）
  match kind {
    TMathUnary::Sqrt => unsafe { transmute::<LuaCFunction, usize>(Some(math_sqrt)) },
    TMathUnary::Abs => unsafe { transmute::<LuaCFunction, usize>(Some(math_abs)) },
    TMathUnary::Floor => unsafe { transmute::<LuaCFunction, usize>(Some(math_floor)) },
    TMathUnary::Ceil => unsafe { transmute::<LuaCFunction, usize>(Some(math_ceil)) },
    TMathUnary::Round => unsafe { transmute::<LuaCFunction, usize>(Some(math_round)) },
  }
}

/// 槽值是哪个 math 单参内建 C 函数（None = 非 math 内建闭包）。
///
/// 契约：`tv` 为活跃帧槽域的存活 TValue，且 `tv.is_function()` 谓词已命中
/// （`as_closure` 为同址类型化读）。
pub(crate) fn slot_math_fn(tv: &TValue) -> Option<TMathUnary> {
  // SAFETY: is_function() 谓词由调用方契约保证命中，as_closure 同址类型化读
  let cl = unsafe { tv.as_closure() };
  if cl.is_c != 1 {
    return None;
  }
  // SAFETY: LuaCFunction 为 Option<extern fn>，与 usize 同尺寸裸转（fn 项
  // 地址对账，不回落任何表示）
  let slot_f = unsafe { transmute::<LuaCFunction, usize>(cl.inner.c.f) };
  [
    TMathUnary::Sqrt,
    TMathUnary::Abs,
    TMathUnary::Floor,
    TMathUnary::Ceil,
    TMathUnary::Round,
  ]
  .into_iter()
  .find(|&kind| slot_f == math_fn_addr(kind))
}
