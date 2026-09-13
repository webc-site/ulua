use crate::{enums::ir_condition::IrCondition, macros::codegen_assert::CODEGEN_ASSERT};

pub fn compare_i64_i64_ir_condition(a: i64, b: i64, cond: IrCondition) -> bool {
  match cond {
    IrCondition::Equal => a == b,
    IrCondition::NotEqual => a != b,
    IrCondition::Less => a < b,
    IrCondition::NotLess => !(a < b),
    IrCondition::LessEqual => a <= b,
    IrCondition::NotLessEqual => !(a <= b),
    IrCondition::Greater => a > b,
    IrCondition::NotGreater => !(a > b),
    IrCondition::GreaterEqual => a >= b,
    IrCondition::NotGreaterEqual => !(a >= b),
    IrCondition::UnsignedLess => (a as u64) < (b as u64),
    IrCondition::UnsignedLessEqual => (a as u64) <= (b as u64),
    IrCondition::UnsignedGreater => (a as u64) > (b as u64),
    IrCondition::UnsignedGreaterEqual => (a as u64) >= (b as u64),
    _ => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      false
    }
  }
}
