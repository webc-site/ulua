use crate::{
  enums::{condition_x_64::ConditionX64, ir_condition::IrCondition},
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// cpp `EmitCommonX64.cpp:98` `getConditionInt`：`IrCondition` → x64 整数条件码。
/// 以 `IrCondition`（`#[repr(u8)]`，判别式 0..=13）为下标的编译期定表；仅 `Count`
/// （下标越界）落到 `None` 分支（等价旧 `_ =>` 臂，含 `CODEGEN_ASSERT`）。哨兵
/// `Count` + 下方 const 自检保证 14 项全部赋值。
const IR_TO_X64_INT: [ConditionX64; IrCondition::Count as usize] = {
  let mut t = [ConditionX64::Count; IrCondition::Count as usize];
  t[IrCondition::Equal as usize] = ConditionX64::Equal;
  t[IrCondition::NotEqual as usize] = ConditionX64::NotEqual;
  t[IrCondition::Less as usize] = ConditionX64::Less;
  t[IrCondition::NotLess as usize] = ConditionX64::NotLess;
  t[IrCondition::LessEqual as usize] = ConditionX64::LessEqual;
  t[IrCondition::NotLessEqual as usize] = ConditionX64::NotLessEqual;
  t[IrCondition::Greater as usize] = ConditionX64::Greater;
  t[IrCondition::NotGreater as usize] = ConditionX64::NotGreater;
  t[IrCondition::GreaterEqual as usize] = ConditionX64::GreaterEqual;
  t[IrCondition::NotGreaterEqual as usize] = ConditionX64::NotGreaterEqual;
  t[IrCondition::UnsignedLess as usize] = ConditionX64::Below;
  t[IrCondition::UnsignedLessEqual as usize] = ConditionX64::BelowEqual;
  t[IrCondition::UnsignedGreater as usize] = ConditionX64::Above;
  t[IrCondition::UnsignedGreaterEqual as usize] = ConditionX64::AboveEqual;
  t
};

const _: () = {
  let mut i = 0;
  while i < IR_TO_X64_INT.len() {
    assert!(
      IR_TO_X64_INT[i] as u32 != ConditionX64::Count as u32,
      "IR_TO_X64_INT 存在未赋值的哨兵项"
    );
    i += 1;
  }
};

pub fn get_condition_int(cond: IrCondition) -> ConditionX64 {
  match IR_TO_X64_INT.get(cond as usize) {
    Some(&value) => value,
    None => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      ConditionX64::Zero
    }
  }
}
