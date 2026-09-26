use crate::{enums::ir_condition::IrCondition, macros::codegen_assert::CODEGEN_ASSERT};

/// cpp `IrUtils.h:142` `getNegatedCondition`：`IrCondition` → 取反条件。
/// 以 `IrCondition`（`#[repr(u8)]`，判别式 0..=13）为下标的编译期定表；仅 `Count`
/// （下标越界）落到 `None` 分支（等价旧 `_ =>` 臂，含 `CODEGEN_ASSERT`）。
const IR_NEGATED: [IrCondition; IrCondition::Count as usize] = {
  let mut t = [IrCondition::Count; IrCondition::Count as usize];
  t[IrCondition::Equal as usize] = IrCondition::NotEqual;
  t[IrCondition::NotEqual as usize] = IrCondition::Equal;
  t[IrCondition::Less as usize] = IrCondition::NotLess;
  t[IrCondition::NotLess as usize] = IrCondition::Less;
  t[IrCondition::LessEqual as usize] = IrCondition::NotLessEqual;
  t[IrCondition::NotLessEqual as usize] = IrCondition::LessEqual;
  t[IrCondition::Greater as usize] = IrCondition::NotGreater;
  t[IrCondition::NotGreater as usize] = IrCondition::Greater;
  t[IrCondition::GreaterEqual as usize] = IrCondition::NotGreaterEqual;
  t[IrCondition::NotGreaterEqual as usize] = IrCondition::GreaterEqual;
  t[IrCondition::UnsignedLess as usize] = IrCondition::UnsignedGreaterEqual;
  t[IrCondition::UnsignedLessEqual as usize] = IrCondition::UnsignedGreater;
  t[IrCondition::UnsignedGreater as usize] = IrCondition::UnsignedLessEqual;
  t[IrCondition::UnsignedGreaterEqual as usize] = IrCondition::UnsignedLess;
  t
};

/// 取反必为对合（negate(negate(c)) == c）且无哨兵残留：编译期逐指针核验。
const _: () = {
  let mut i = 0;
  while i < IR_NEGATED.len() {
    let n = IR_NEGATED[i] as u8;
    assert!(
      n < IR_NEGATED.len() as u8,
      "IR_NEGATED 存在未赋值或越界的哨兵项"
    );
    assert!(
      IR_NEGATED[n as usize] as u8 == i as u8,
      "IR_NEGATED 不满足对合性 negate(negate(c)) == c"
    );
    i += 1;
  }
};

pub fn get_negated_condition_ir_condition(cond: IrCondition) -> IrCondition {
  match IR_NEGATED.get(cond as usize) {
    Some(&value) => value,
    None => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      IrCondition::Count
    }
  }
}
