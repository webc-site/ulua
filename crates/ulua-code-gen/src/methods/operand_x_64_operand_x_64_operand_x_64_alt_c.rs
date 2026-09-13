use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};
impl OperandX64 {
  pub fn operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    size: SizeX64,
    index: RegisterX64,
    scale: u8,
    base: RegisterX64,
    disp: i32,
  ) -> Self {
    OperandX64 {
      cat: CategoryX64::Mem,
      index,
      base,
      mem_size: size,
      scale,
      imm: disp,
    }
  }
}
