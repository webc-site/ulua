use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn movzx(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64("movzx", OperandX64::reg(lhs), rhs);
    }

    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };

    if !(size == SizeX64::Byte || size == SizeX64::Word) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64_operand_x_64(lhs, rhs);
    self.place(0x0f);
    self.place(if size == SizeX64::Byte { 0xb6 } else { 0xb7 });
    self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
    self.commit();
  }
}
