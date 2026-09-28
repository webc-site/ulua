use crate::enums::{condition_a_64::ConditionA64, ir_condition::IrCondition};

crate::cond_lookup_table! {
  /// cpp `IrLoweringA64.cpp:123` `getConditionInt64`：`IrCondition` → A64 int64 全量程条件。
  /// 以 `IrCondition`（`#[repr(u8)]`，判别式 0..=13）为下标的编译期定表；仅 `Count`
  /// （下标越界）落到回退臂（等价旧 `_ =>` 臂，含 `CODEGEN_ASSERT`）。填充哨兵
  /// `Count`，宏生成的 const 自检确保 14 项全部显式赋值、无遗漏。
  #[inline]
  pub fn get_condition_int_64(cond: IrCondition) -> ConditionA64 {
    table: IR_TO_A64_INT64,
    len: IrCondition::Count as usize,
    sentinel: ConditionA64::Count,
    assert: "Unexpected condition code",
    fallback: ConditionA64::Always,
    entries: {
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
    }
  }
}
