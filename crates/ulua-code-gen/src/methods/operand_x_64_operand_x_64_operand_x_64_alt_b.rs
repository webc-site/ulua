use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

impl OperandX64 {
  pub fn operand_x_64_i32(imm: i32) -> Self {
    OperandX64 {
      cat: CategoryX64::Imm,
      index: RegisterX64 { bits: 0xFF },
      base: RegisterX64 { bits: 0xFF },
      mem_size: SizeX64::None,
      scale: 1,
      imm,
    }
  }
}
