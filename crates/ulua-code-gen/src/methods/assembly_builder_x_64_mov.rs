use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::op_plus_reg::op_plus_reg,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn mov(&mut self, lhs: OperandX64, rhs: OperandX64) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64("mov", lhs, rhs);
    }

    if lhs.cat == CategoryX64::Reg && rhs.cat == CategoryX64::Imm {
      let size = lhs.base.size();

      self.place_rex_register_x_64(lhs.base);

      if size == SizeX64::Byte {
        self.place(op_plus_reg(0xb0, lhs.base.index()));
        self.place_imm_8(rhs.imm);
      } else if size == SizeX64::Word {
        self.place(0x66);
        self.place(op_plus_reg(0xb8, lhs.base.index()));
        self.place_imm_16(rhs.imm as i16);
      } else if size == SizeX64::Dword {
        self.place(op_plus_reg(0xb8, lhs.base.index()));
        self.place_imm_32(rhs.imm);
      } else {
        // qword
        self.place(op_plus_reg(0xb8, lhs.base.index()));
        self.place_imm_64(rhs.imm as i64);
      }
    } else if lhs.cat == CategoryX64::Mem && rhs.cat == CategoryX64::Imm {
      let size = lhs.mem_size;

      self.place_rex_operand_x_64(lhs);

      if size == SizeX64::Byte {
        self.place(0xc6);
        self.place_mod_reg_mem(lhs, 0, 1);
        self.place_imm_8(rhs.imm);
      } else if size == SizeX64::Word {
        self.place(0x66);
        self.place(0xc7);
        self.place_mod_reg_mem(lhs, 0, 2);
        self.place_imm_16(rhs.imm as i16);
      } else {
        // dword or qword: both encoded with imm32 in this routine
        self.place(0xc7);
        self.place_mod_reg_mem(lhs, 0, 4);
        self.place_imm_32(rhs.imm);
      }
    } else if lhs.cat == CategoryX64::Reg
      && (rhs.cat == CategoryX64::Reg || rhs.cat == CategoryX64::Mem)
    {
      self.place_binary_reg_and_reg_mem(lhs, rhs, 0x8a, 0x8b);
    } else if lhs.cat == CategoryX64::Mem && rhs.cat == CategoryX64::Reg {
      self.place_binary_reg_mem_and_reg(lhs, rhs, 0x88, 0x89);
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.commit();
  }
}
