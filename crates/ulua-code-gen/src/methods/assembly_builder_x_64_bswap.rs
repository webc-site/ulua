use crate::{
  enums::size_x_64::SizeX64,
  macros::op_plus_reg::op_plus_reg,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};
impl AssemblyBuilderX64 {
  pub fn bswap(&mut self, dst: RegisterX64) {
    if self.log_text {
      self.log_c_char_operand_x_64("bswap", OperandX64::reg(dst));
    }

    if !(dst.size() == SizeX64::Dword || dst.size() == SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(dst);
    self.place(0x0f);
    self.place(op_plus_reg(0xc8, dst.index()));
    self.commit();
  }
}
