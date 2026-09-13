use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn place_unary_mod_reg_mem(
    &mut self,
    name: &str,
    op: OperandX64,
    code8: u8,
    code: u8,
    opreg: u8,
  ) {
    if self.log_text {
      self.log_c_char_operand_x_64(name, op);
    }

    if !(op.cat == CategoryX64::Reg || op.cat == CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = if op.cat == CategoryX64::Reg {
      op.base.size()
    } else {
      op.mem_size
    };

    if !(size == SizeX64::Byte || size == SizeX64::Dword || size == SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_operand_x_64(op);
    self.place(if size == SizeX64::Byte { code8 } else { code });
    self.place_mod_reg_mem(op, opreg, 0);

    self.commit();
  }
}
