use crate::enums::condition_x_64::ConditionX64;

crate::cond_lookup_table! {
  /// cpp `EmitCommonX64.h` `getNegatedCondition`：`ConditionX64` → 取反条件码。
  /// 以 `ConditionX64`（`#[repr(u8)]`，判别式 0..=25）为下标的编译期定表；仅 `Count`
  /// （下标越界）落到回退臂（等价旧 `ConditionX64::Count =>` 臂，含 `CODEGEN_ASSERT`）。
  /// 哨兵 `Count` + 宏生成的 const 自检保证 26 项全部赋值。
  pub fn get_negated_condition(cond: ConditionX64) -> ConditionX64 {
    table: X64_NEGATED,
    len: ConditionX64::Count as usize,
    sentinel: ConditionX64::Count,
    assert: "invalid ConditionX64 value",
    fallback: ConditionX64::Count,
    entries: {
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
    }
  }
}
