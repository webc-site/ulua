use crate::enums::{condition_a_64::ConditionA64, ir_condition::IrCondition};

#[inline]
pub fn get_condition_int_64(cond: IrCondition) -> ConditionA64 {
  match cond {
    IrCondition::Equal => ConditionA64::Equal,
    IrCondition::NotEqual => ConditionA64::NotEqual,
    IrCondition::Less => ConditionA64::Less,
    IrCondition::NotLess => ConditionA64::GreaterEqual,
    IrCondition::LessEqual => ConditionA64::LessEqual,
    IrCondition::NotLessEqual => ConditionA64::Greater,
    IrCondition::Greater => ConditionA64::Greater,
    IrCondition::NotGreater => ConditionA64::LessEqual,
    IrCondition::GreaterEqual => ConditionA64::GreaterEqual,
    IrCondition::NotGreaterEqual => ConditionA64::Less,
    IrCondition::UnsignedLess => ConditionA64::CarryClear,
    IrCondition::UnsignedLessEqual => ConditionA64::UnsignedLessEqual,
    IrCondition::UnsignedGreater => ConditionA64::UnsignedGreater,
    IrCondition::UnsignedGreaterEqual => ConditionA64::CarrySet,
    _ => {
      debug_assert!(false, "Unexpected condition code");
      ConditionA64::Always
    }
  }
}
