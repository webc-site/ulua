use crate::{
  enums::{condition_a_64::ConditionA64, ir_condition::IrCondition},
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// cpp `IrLoweringA64.cpp:64` `getConditionInt`：`IrCondition` → A64 小整型比较条件
/// （Less/NotLess 用 MI/PL，即仅检 N 标志）。以 `IrCondition`（`#[repr(u8)]`，判别式
/// 0..=13）为下标的编译期定表；仅 `Count`（下标越界）落到 `None` 分支（等价旧 `_ =>`
/// 臂，含 `CODEGEN_ASSERT`）。哨兵 `Count` + 下方 const 自检保证 14 项全部赋值。
const IR_TO_A64_INT: [ConditionA64; IrCondition::Count as usize] = {
  let mut t = [ConditionA64::Count; IrCondition::Count as usize];
  t[IrCondition::Equal as usize] = ConditionA64::Equal;
  t[IrCondition::NotEqual as usize] = ConditionA64::NotEqual;
  t[IrCondition::Less as usize] = ConditionA64::Minus;
  t[IrCondition::NotLess as usize] = ConditionA64::Plus;
  t[IrCondition::LessEqual as usize] = ConditionA64::LessEqual;
  t[IrCondition::NotLessEqual as usize] = ConditionA64::Greater;
  t[IrCondition::Greater as usize] = ConditionA64::Greater;
  t[IrCondition::NotGreater as usize] = ConditionA64::LessEqual;
  t[IrCondition::GreaterEqual as usize] = ConditionA64::GreaterEqual;
  t[IrCondition::NotGreaterEqual as usize] = ConditionA64::Less;
  t[IrCondition::UnsignedLess as usize] = ConditionA64::CarryClear;
  t[IrCondition::UnsignedLessEqual as usize] = ConditionA64::UnsignedLessEqual;
  t[IrCondition::UnsignedGreater as usize] = ConditionA64::UnsignedGreater;
  t[IrCondition::UnsignedGreaterEqual as usize] = ConditionA64::CarrySet;
  t
};

const _: () = {
  let mut i = 0;
  while i < IR_TO_A64_INT.len() {
    assert!(
      IR_TO_A64_INT[i] as u32 != ConditionA64::Count as u32,
      "IR_TO_A64_INT 存在未赋值的哨兵项"
    );
    i += 1;
  }
};

#[inline]
pub fn get_condition_int(cond: IrCondition) -> ConditionA64 {
  match IR_TO_A64_INT.get(cond as usize) {
    Some(&value) => value,
    None => {
      CODEGEN_ASSERT!(false, "Unexpected condition code");
      ConditionA64::Always
    }
  }
}
