use core::cmp::Ordering;

use crate::{enums::ir_condition::IrCondition, macros::codegen_assert::CODEGEN_ASSERT};
pub fn compare_f64_f64_ir_condition(a: f64, b: f64, cond: IrCondition) -> bool {
  // Note: the C++ source uses redundant bool() casts to work around an invalid MSVC
  // optimization that violated IEEE754 comparison semantics; Rust has no such issue.
  match cond {
    IrCondition::Equal => a == b,
    IrCondition::NotEqual => a != b,
    IrCondition::Less => a < b,
    IrCondition::NotLess => !matches!(a.partial_cmp(&b), Some(Ordering::Less)),
    IrCondition::LessEqual => a <= b,
    IrCondition::NotLessEqual => {
      !matches!(a.partial_cmp(&b), Some(Ordering::Less | Ordering::Equal))
    }
    IrCondition::Greater => a > b,
    IrCondition::NotGreater => !matches!(a.partial_cmp(&b), Some(Ordering::Greater)),
    IrCondition::GreaterEqual => a >= b,
    IrCondition::NotGreaterEqual => {
      !matches!(a.partial_cmp(&b), Some(Ordering::Greater | Ordering::Equal))
    }
    _ => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      false
    }
  }
}
