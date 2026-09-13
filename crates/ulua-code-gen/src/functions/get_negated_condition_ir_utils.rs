use crate::{enums::ir_condition::IrCondition, macros::codegen_assert::CODEGEN_ASSERT};

pub fn get_negated_condition_ir_condition(cond: IrCondition) -> IrCondition {
  match cond {
    IrCondition::Equal => IrCondition::NotEqual,
    IrCondition::NotEqual => IrCondition::Equal,
    IrCondition::Less => IrCondition::NotLess,
    IrCondition::NotLess => IrCondition::Less,
    IrCondition::LessEqual => IrCondition::NotLessEqual,
    IrCondition::NotLessEqual => IrCondition::LessEqual,
    IrCondition::Greater => IrCondition::NotGreater,
    IrCondition::NotGreater => IrCondition::Greater,
    IrCondition::GreaterEqual => IrCondition::NotGreaterEqual,
    IrCondition::NotGreaterEqual => IrCondition::GreaterEqual,
    IrCondition::UnsignedLess => IrCondition::UnsignedGreaterEqual,
    IrCondition::UnsignedLessEqual => IrCondition::UnsignedGreater,
    IrCondition::UnsignedGreater => IrCondition::UnsignedLessEqual,
    IrCondition::UnsignedGreaterEqual => IrCondition::UnsignedLess,
    _ => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      IrCondition::Count
    }
  }
}
