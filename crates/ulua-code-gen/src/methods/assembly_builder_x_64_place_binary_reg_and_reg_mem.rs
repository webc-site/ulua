use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn place_binary_reg_and_reg_mem(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    code8: u8,
    code: u8,
  ) {
    if !(lhs.cat == CategoryX64::Reg
      && (rhs.cat == CategoryX64::Reg || rhs.cat == CategoryX64::Mem))
    {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };
    if !(lhs.base.size() == size) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if !(size == SizeX64::Byte
      || size == SizeX64::Word
      || size == SizeX64::Dword
      || size == SizeX64::Qword)
    {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if size == SizeX64::Word {
      self.place(0x66);
    }

    self.place_rex_register_x_64_operand_x_64(lhs.base, rhs);
    self.place(if size == SizeX64::Byte { code8 } else { code });
    self.place_reg_and_mod_reg_mem(lhs, rhs, 0);

    self.commit();
  }
}
