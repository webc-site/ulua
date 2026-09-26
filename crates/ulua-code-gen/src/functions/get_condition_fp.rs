use crate::enums::{condition_a_64::ConditionA64, ir_condition::IrCondition};

/// cpp `IrLoweringA64.cpp:24` `getConditionFP`：`IrCondition` → A64 浮点比较条件。
/// 仅前 10 个（Equal..NotGreaterEqual，判别式 0..=9）有映射，无符号条件与 `Count`
/// 一律返回 `Always`（等价旧 `_ => ConditionA64::Always`，本函数无断言）。以判别式为
/// 下标的编译期定表 + `unwrap_or(Always)` 覆盖越界项。哨兵 `Count` + 下方 const 自检
/// 保证 10 项全部赋值。
const IR_TO_A64_FP: [ConditionA64; 10] = {
  let mut t = [ConditionA64::Count; 10];
  t[IrCondition::Equal as usize] = ConditionA64::Equal;
  t[IrCondition::NotEqual as usize] = ConditionA64::NotEqual;
  t[IrCondition::Less as usize] = ConditionA64::Minus;
  t[IrCondition::NotLess as usize] = ConditionA64::Plus;
  t[IrCondition::LessEqual as usize] = ConditionA64::UnsignedLessEqual;
  t[IrCondition::NotLessEqual as usize] = ConditionA64::UnsignedGreater;
  t[IrCondition::Greater as usize] = ConditionA64::Greater;
  t[IrCondition::NotGreater as usize] = ConditionA64::LessEqual;
  t[IrCondition::GreaterEqual as usize] = ConditionA64::GreaterEqual;
  t[IrCondition::NotGreaterEqual as usize] = ConditionA64::Less;
  t
};

const _: () = {
  let mut i = 0;
  while i < IR_TO_A64_FP.len() {
    assert!(
      IR_TO_A64_FP[i] as u32 != ConditionA64::Count as u32,
      "IR_TO_A64_FP 存在未赋值的哨兵项"
    );
    i += 1;
  }
};

#[inline]
pub fn get_condition_fp(cond: IrCondition) -> ConditionA64 {
  IR_TO_A64_FP
    .get(cond as usize)
    .copied()
    .unwrap_or(ConditionA64::Always)
}
