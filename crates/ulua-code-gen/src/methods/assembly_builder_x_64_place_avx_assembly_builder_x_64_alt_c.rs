use crate::{
  enums::category_x_64::CategoryX64,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};
impl AssemblyBuilderX64 {
  pub fn place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
    &mut self,
    name: &str,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    code: u8,
    set_w: bool,
    mode: u8,
    prefix: u8,
  ) {
    if !(dst.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(src1.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(src2.cat == CategoryX64::Reg || src2.cat == CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64_operand_x_64(name, dst, src1, src2);
    }

    self.place_vex(dst, src1, src2, set_w, mode, prefix);
    self.place(code);
    self.place_reg_and_mod_reg_mem(dst, src2, 0);
    self.commit();
  }
}
