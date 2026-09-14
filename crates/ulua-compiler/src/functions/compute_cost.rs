use core::slice::from_raw_parts;

use crate::functions::cost_model::{K_MAX_COST_VARS, K_VAR_DISCOUNT_BITS};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn compute_cost(model: u64, vars_const: *const bool, var_count: usize) -> i32 {
  let mut cost = (model & 0x7f) as i32;

  // don't apply discounts to what is likely a saturated sum
  if cost == 0x7f {
    return cost;
  }

  if var_count > 0 {
    let vars_const = unsafe { from_raw_parts(vars_const, var_count.min(K_MAX_COST_VARS)) };
    for (i, &is_const) in vars_const.iter().enumerate() {
      let discount =
        ((model >> ((i * K_VAR_DISCOUNT_BITS + K_VAR_DISCOUNT_BITS) as u32)) & 0x7f) as i32;
      cost -= discount * (is_const as i32);
    }
  }

  cost
}
