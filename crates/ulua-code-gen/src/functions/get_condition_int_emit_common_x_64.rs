use crate::enums::{condition_x_64::ConditionX64, ir_condition::IrCondition};

crate::cond_lookup_table! {
  /// cpp `EmitCommonX64.cpp:98` `getConditionInt`：`IrCondition` → x64 整数条件码。
  /// 以 `IrCondition`（`#[repr(u8)]`，判别式 0..=13）为下标的编译期定表；仅 `Count`
  /// （下标越界）落到回退臂（等价旧 `_ =>` 臂，含 `CODEGEN_ASSERT`）。哨兵
  /// `Count` + 宏生成的 const 自检保证 14 项全部赋值。
  pub fn get_condition_int(cond: IrCondition) -> ConditionX64 {
    table: IR_TO_X64_INT,
    len: IrCondition::Count as usize,
    sentinel: ConditionX64::Count,
    assert: "Unsupported condition",
    fallback: ConditionX64::Zero,
    entries: {
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
    }
  }
}
