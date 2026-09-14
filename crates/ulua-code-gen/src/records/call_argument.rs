use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{ir_op::IrOp, operand_x_64::OperandX64, register_x_64::RegisterX64},
};
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CallArgument {
  pub target_size: SizeX64,
  pub source: OperandX64,
  pub source_op: IrOp,
  pub target: OperandX64,
  pub candidate: bool,
}

impl Default for CallArgument {
  fn default() -> Self {
    Self {
      target_size: SizeX64::None,
      source: OperandX64 {
        cat: CategoryX64::Reg,
        index: RegisterX64::NOREG,
        base: RegisterX64::NOREG,
        mem_size: SizeX64::None,
        scale: 1,
        imm: 0,
      },
      source_op: IrOp::default(),
      target: OperandX64 {
        cat: CategoryX64::Reg,
        index: RegisterX64::NOREG,
        base: RegisterX64::NOREG,
        mem_size: SizeX64::None,
        scale: 1,
        imm: 0,
      },
      candidate: true,
    }
  }
}
