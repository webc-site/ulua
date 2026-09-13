use crate::{
  enums::{condition_x_64::ConditionX64, ir_condition::IrCondition},
  macros::codegen_assert::CODEGEN_ASSERT,
};

pub fn get_condition_int(cond: IrCondition) -> ConditionX64 {
  match cond {
    IrCondition::Equal => ConditionX64::Equal,
    IrCondition::NotEqual => ConditionX64::NotEqual,
    IrCondition::Less => ConditionX64::Less,
    IrCondition::NotLess => ConditionX64::NotLess,
    IrCondition::LessEqual => ConditionX64::LessEqual,
    IrCondition::NotLessEqual => ConditionX64::NotLessEqual,
    IrCondition::Greater => ConditionX64::Greater,
    IrCondition::NotGreater => ConditionX64::NotGreater,
    IrCondition::GreaterEqual => ConditionX64::GreaterEqual,
    IrCondition::NotGreaterEqual => ConditionX64::NotGreaterEqual,
    IrCondition::UnsignedLess => ConditionX64::Below,
    IrCondition::UnsignedLessEqual => ConditionX64::BelowEqual,
    IrCondition::UnsignedGreater => ConditionX64::Above,
    IrCondition::UnsignedGreaterEqual => ConditionX64::AboveEqual,
    _ => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      ConditionX64::Zero
    }
  }
}
