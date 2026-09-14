use crate::{
  enums::size_x_64::SizeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn bsf(&mut self, dst: RegisterX64, src: OperandX64) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64("bsf", OperandX64::reg(dst), src);
    }

    if !(dst.size() == SizeX64::Dword || dst.size() == SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64_operand_x_64(dst, src);
    self.place(0x0f);
    self.place(0xbc);
    self.place_reg_and_mod_reg_mem(OperandX64::reg(dst), src, 0);
    self.commit();
  }
}
