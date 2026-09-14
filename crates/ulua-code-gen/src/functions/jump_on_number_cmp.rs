use core::mem::swap;

use crate::{
  enums::{category_x_64::CategoryX64, condition_x_64::ConditionX64, ir_condition::IrCondition},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, label::Label, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

pub fn jump_on_number_cmp(
  build: &mut AssemblyBuilderX64,
  tmp: RegisterX64,
  mut lhs: OperandX64,
  mut rhs: OperandX64,
  cond: IrCondition,
  label: &mut Label,
  float_precision: bool,
) {
  if matches!(
    cond,
    IrCondition::Greater
      | IrCondition::GreaterEqual
      | IrCondition::NotGreater
      | IrCondition::NotGreaterEqual
  ) {
    swap(&mut lhs, &mut rhs);
  }

  if float_precision {
    if rhs.cat == CategoryX64::Reg {
      build.vucomiss(rhs, lhs);
    } else {
      build.vmovss_operand_x_64_operand_x_64(OperandX64::reg(tmp), rhs);
      build.vucomiss(OperandX64::reg(tmp), lhs);
    }
  } else if rhs.cat == CategoryX64::Reg {
    build.vucomisd(rhs, lhs);
  } else {
    build.vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp), rhs);
    build.vucomisd(OperandX64::reg(tmp), lhs);
  }

  match cond {
    IrCondition::NotLessEqual | IrCondition::NotGreaterEqual => {
      build.jcc(ConditionX64::NotAboveEqual, label);
    }
    IrCondition::LessEqual | IrCondition::GreaterEqual => {
      build.jcc(ConditionX64::AboveEqual, label);
    }
    IrCondition::NotLess | IrCondition::NotGreater => {
      build.jcc(ConditionX64::NotAbove, label);
    }
    IrCondition::Less | IrCondition::Greater => {
      build.jcc(ConditionX64::Above, label);
    }
    IrCondition::NotEqual => {
      build.jcc(ConditionX64::NotZero, label);
      build.jcc(ConditionX64::Parity, label);
    }
    _ => crate::macros::codegen_assert::CODEGEN_ASSERT!(false),
  }
}
