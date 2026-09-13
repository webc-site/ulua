use crate::{enums::condition_x_64::ConditionX64, macros::codegen_assert::CODEGEN_ASSERT};

pub fn get_inverse_condition(cond: ConditionX64) -> ConditionX64 {
  match cond {
    ConditionX64::Below => ConditionX64::Above,
    ConditionX64::BelowEqual => ConditionX64::AboveEqual,
    ConditionX64::Above => ConditionX64::Below,
    ConditionX64::AboveEqual => ConditionX64::BelowEqual,
    ConditionX64::Equal => ConditionX64::Equal,
    ConditionX64::Less => ConditionX64::Greater,
    ConditionX64::LessEqual => ConditionX64::GreaterEqual,
    ConditionX64::Greater => ConditionX64::Less,
    ConditionX64::GreaterEqual => ConditionX64::LessEqual,
    ConditionX64::NotBelow => ConditionX64::NotAbove,
    ConditionX64::NotBelowEqual => ConditionX64::NotAboveEqual,
    ConditionX64::NotAbove => ConditionX64::NotBelow,
    ConditionX64::NotAboveEqual => ConditionX64::NotBelowEqual,
    ConditionX64::NotEqual => ConditionX64::NotEqual,
    ConditionX64::NotLess => ConditionX64::NotGreater,
    ConditionX64::NotLessEqual => ConditionX64::NotGreaterEqual,
    ConditionX64::NotGreater => ConditionX64::NotLess,
    ConditionX64::NotGreaterEqual => ConditionX64::NotLessEqual,
    _ => {
      CODEGEN_ASSERT!(false, "invalid ConditionX64 value for getInverseCondition");
      ConditionX64::Count
    }
  }
}
