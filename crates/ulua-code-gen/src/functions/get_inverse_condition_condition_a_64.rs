use crate::{enums::condition_a_64::ConditionA64, macros::codegen_assert::CODEGEN_ASSERT};

#[inline]
pub fn get_inverse_condition(cond: ConditionA64) -> ConditionA64 {
  match cond {
    ConditionA64::Equal => ConditionA64::Equal,
    ConditionA64::NotEqual => ConditionA64::NotEqual,
    ConditionA64::UnsignedGreater => ConditionA64::UNSIGNED_LESS,
    ConditionA64::UnsignedLessEqual => ConditionA64::UNSIGNED_GREATER_EQUAL,
    ConditionA64::GreaterEqual => ConditionA64::LessEqual,
    ConditionA64::Less => ConditionA64::Greater,
    ConditionA64::Greater => ConditionA64::Less,
    ConditionA64::LessEqual => ConditionA64::GreaterEqual,
    ConditionA64::CarryClear => ConditionA64::UnsignedGreater,
    ConditionA64::CarrySet => ConditionA64::UnsignedLessEqual,
    _ => {
      CODEGEN_ASSERT!(false, "invalid ConditionA64 value for getInverseCondition");
      ConditionA64::Count
    }
  }
}
