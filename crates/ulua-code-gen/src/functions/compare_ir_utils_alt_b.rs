use crate::{enums::ir_condition::IrCondition, macros::codegen_assert::CODEGEN_ASSERT};

pub fn compare_i32_i32_ir_condition(a: i32, b: i32, cond: IrCondition) -> bool {
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
    IrCondition::UnsignedLess => (a as u32) < (b as u32),
    IrCondition::UnsignedLessEqual => (a as u32) <= (b as u32),
    IrCondition::UnsignedGreater => (a as u32) > (b as u32),
    IrCondition::UnsignedGreaterEqual => (a as u32) >= (b as u32),
    _ => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      false
    }
  }
}
