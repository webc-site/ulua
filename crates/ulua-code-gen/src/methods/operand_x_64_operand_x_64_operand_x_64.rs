use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

impl OperandX64 {
  pub fn operand_x_64_register_x_64(reg: RegisterX64) -> Self {
    Self {
      cat: CategoryX64::Reg,
      index: RegisterX64::NOREG,
      base: reg,
      mem_size: SizeX64::None,
      scale: 1,
      imm: 0,
    }
  }
}
