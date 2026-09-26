use crate::functions::cost_model::{K_MAX_COST_VARS, K_VAR_DISCOUNT_BITS};

/// 折扣字段掩码：8 位槽低 7 位为有效折扣（0x7f 同时是饱和哨兵值）
const K_DISCOUNT_MASK: u64 = 0x7f;

/// cpp/Compiler/src/CostModel.cpp `computeCost(model, varsConst, varCount)`（:444）。
/// `vars_const[i]` 表示第 i 个形参是否为编译期常量（常量可享对应槽位的折扣）。
/// C++ 侧的 `const bool* + size_t` 在 Rust 收成切片借用：空切片即 `nullptr, 0`。
pub fn compute_cost(model: u64, vars_const: &[bool]) -> i32 {
  let mut cost = (model & K_DISCOUNT_MASK) as i32;

  // 很可能已是饱和求和时不再套用折扣
  if cost == K_DISCOUNT_MASK as i32 {
    return cost;
  }

  for (i, &is_const) in vars_const.iter().take(K_MAX_COST_VARS).enumerate() {
    let discount = ((model >> ((i + 1) * K_VAR_DISCOUNT_BITS)) & K_DISCOUNT_MASK) as i32;
    cost -= discount * i32::from(is_const);
  }

  cost
}
