use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn place_shift(&mut self, name: &str, lhs: OperandX64, rhs: OperandX64, opreg: u8) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64(name, lhs, rhs);
    }

    let cl = RegisterX64 {
      bits: (1u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Byte as u8,
    };

    if !(lhs.cat == CategoryX64::Reg || lhs.cat == CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(rhs.cat == CategoryX64::Imm || (rhs.cat == CategoryX64::Reg && rhs.base == cl)) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = lhs.base.size();

    self.place_rex_register_x_64(lhs.base);

    if rhs.cat == CategoryX64::Imm && rhs.imm == 1 {
      self.place(if size == SizeX64::Byte { 0xd0 } else { 0xd1 });
      self.place_mod_reg_mem(lhs, opreg, 0);
    } else if rhs.cat == CategoryX64::Imm {
      if !((rhs.imm as i8) as i32 == rhs.imm) {
        ulua_common::LUAU_DEBUGBREAK!();
      }

      self.place(if size == SizeX64::Byte { 0xc0 } else { 0xc1 });
      self.place_mod_reg_mem(lhs, opreg, 1);
      self.place_imm_8(rhs.imm);
    } else {
      self.place(if size == SizeX64::Byte { 0xd2 } else { 0xd3 });
      self.place_mod_reg_mem(lhs, opreg, 0);
    }

    self.commit();
  }
}
