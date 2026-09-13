use crate::{enums::condition_x_64::ConditionX64, macros::codegen_assert::CODEGEN_ASSERT};

pub fn get_negated_condition(cond: ConditionX64) -> ConditionX64 {
  match cond {
    ConditionX64::Overflow => ConditionX64::NoOverflow,
    ConditionX64::NoOverflow => ConditionX64::Overflow,
    ConditionX64::Carry => ConditionX64::NoCarry,
    ConditionX64::NoCarry => ConditionX64::Carry,
    ConditionX64::Below => ConditionX64::NotBelow,
    ConditionX64::BelowEqual => ConditionX64::NotBelowEqual,
    ConditionX64::Above => ConditionX64::NotAbove,
    ConditionX64::AboveEqual => ConditionX64::NotAboveEqual,
    ConditionX64::Equal => ConditionX64::NotEqual,
    ConditionX64::Less => ConditionX64::NotLess,
    ConditionX64::LessEqual => ConditionX64::NotLessEqual,
    ConditionX64::Greater => ConditionX64::NotGreater,
    ConditionX64::GreaterEqual => ConditionX64::NotGreaterEqual,
    ConditionX64::NotBelow => ConditionX64::Below,
    ConditionX64::NotBelowEqual => ConditionX64::BelowEqual,
    ConditionX64::NotAbove => ConditionX64::Above,
    ConditionX64::NotAboveEqual => ConditionX64::AboveEqual,
    ConditionX64::NotEqual => ConditionX64::Equal,
    ConditionX64::NotLess => ConditionX64::Less,
    ConditionX64::NotLessEqual => ConditionX64::LessEqual,
    ConditionX64::NotGreater => ConditionX64::Greater,
    ConditionX64::NotGreaterEqual => ConditionX64::GreaterEqual,
    ConditionX64::Zero => ConditionX64::NotZero,
    ConditionX64::NotZero => ConditionX64::Zero,
    ConditionX64::Parity => ConditionX64::NotParity,
    ConditionX64::NotParity => ConditionX64::Parity,
    ConditionX64::Count => {
      CODEGEN_ASSERT!(false, "invalid ConditionX64 value");
      ConditionX64::Count
    }
  }
}
