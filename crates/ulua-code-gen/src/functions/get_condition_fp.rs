use crate::enums::{condition_a_64::ConditionA64, ir_condition::IrCondition};

#[inline]
pub fn get_condition_fp(cond: IrCondition) -> ConditionA64 {
  match cond {
    IrCondition::Equal => ConditionA64::Equal,
    IrCondition::NotEqual => ConditionA64::NotEqual,
    IrCondition::Less => ConditionA64::Minus,
    IrCondition::NotLess => ConditionA64::Plus,
    IrCondition::LessEqual => ConditionA64::UnsignedLessEqual,
    IrCondition::NotLessEqual => ConditionA64::UnsignedGreater,
    IrCondition::Greater => ConditionA64::Greater,
    IrCondition::NotGreater => ConditionA64::LessEqual,
    IrCondition::GreaterEqual => ConditionA64::GreaterEqual,
    IrCondition::NotGreaterEqual => ConditionA64::Less,
    _ => ConditionA64::Always,
  }
}
