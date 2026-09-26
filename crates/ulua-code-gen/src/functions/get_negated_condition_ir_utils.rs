use crate::enums::ir_condition::IrCondition;

crate::cond_lookup_table! {
  /// cpp `IrUtils.h:142` `getNegatedCondition`：`IrCondition` → 取反条件。
  /// 以 `IrCondition`（`#[repr(u8)]`，判别式 0..=13）为下标的编译期定表；仅 `Count`
  /// （下标越界）落到回退臂（等价旧 `_ =>` 臂，含 `CODEGEN_ASSERT`）。
  pub fn get_negated_condition_ir_condition(cond: IrCondition) -> IrCondition {
    table: IR_NEGATED,
    len: IrCondition::Count as usize,
    sentinel: IrCondition::Count,
    assert: "Unsupported condition",
    fallback: IrCondition::Count,
    entries: {
      IrCondition::Equal => IrCondition::NotEqual,
      IrCondition::NotEqual => IrCondition::Equal,
      IrCondition::Less => IrCondition::NotLess,
      IrCondition::NotLess => IrCondition::Less,
      IrCondition::LessEqual => IrCondition::NotLessEqual,
      IrCondition::NotLessEqual => IrCondition::LessEqual,
      IrCondition::Greater => IrCondition::NotGreater,
      IrCondition::NotGreater => IrCondition::Greater,
      IrCondition::GreaterEqual => IrCondition::NotGreaterEqual,
      IrCondition::NotGreaterEqual => IrCondition::GreaterEqual,
      IrCondition::UnsignedLess => IrCondition::UnsignedGreaterEqual,
      IrCondition::UnsignedLessEqual => IrCondition::UnsignedGreater,
      IrCondition::UnsignedGreater => IrCondition::UnsignedLessEqual,
      IrCondition::UnsignedGreaterEqual => IrCondition::UnsignedLess,
    }
  }
}

/// 取反必为对合（negate(negate(c)) == c）：宏的哨兵自检已排除未赋值项，
/// 此处编译期逐值核验对合性。
const _: () = {
  let mut i = 0;
  while i < IR_NEGATED.len() {
    let n = IR_NEGATED[i] as u8;
    assert!(n < IR_NEGATED.len() as u8, "IR_NEGATED 存在越界项");
    assert!(
      IR_NEGATED[n as usize] as u8 == i as u8,
      "IR_NEGATED 不满足对合性 negate(negate(c)) == c"
    );
    i += 1;
  }
};
