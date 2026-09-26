use crate::enums::{condition_a_64::ConditionA64, ir_condition::IrCondition};

crate::cond_lookup_table! {
  /// cpp `IrLoweringA64.cpp:24` `getConditionFP`：`IrCondition` → A64 浮点比较条件。
  /// 仅前 10 个（Equal..NotGreaterEqual，判别式 0..=9）有映射，无符号条件与 `Count`
  /// 一律返回 `Always`（等价旧 `_ => ConditionA64::Always`，本函数无断言）。
  /// 哨兵 `Count` + 宏生成的 const 自检保证 10 项全部赋值。
  #[inline]
  pub fn get_condition_fp(cond: IrCondition) -> ConditionA64 {
    table: IR_TO_A64_FP,
    len: 10,
    sentinel: ConditionA64::Count,
    fallback: ConditionA64::Always,
    entries: {
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
    }
  }
}
