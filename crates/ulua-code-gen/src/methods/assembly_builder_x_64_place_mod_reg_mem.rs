use crate::{
  enums::category_x_64::CategoryX64,
  macros::{mod_rm::mod_rm, sib::sib},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn place_mod_reg_mem(&mut self, rhs: OperandX64, regop: u8, extra_code_bytes: i32) {
    if rhs.cat == CategoryX64::Reg {
      self.place(mod_rm(0b11, regop, rhs.base.index()));
    } else if rhs.cat == CategoryX64::Mem {
      let index = rhs.index;
      let base = rhs.base;

      let mut mod_ = 0b00;

      if rhs.imm != 0 {
        if (rhs.imm as i8) as i32 == rhs.imm {
          mod_ = 0b01;
        } else {
          mod_ = 0b10;
        }
      } else {
        // r13/bp-based addressing requires a displacement
        if (base.index() & 0x7) == 0b101 {
          mod_ = 0b01;
        }
      }

      if index != RegisterX64::NOREG && base != RegisterX64::NOREG {
        self.place(mod_rm(mod_, regop, 0b100));
        self.place(sib(rhs.scale, index.index(), base.index()));

        if mod_ != 0b00 {
          self.place_imm_8_or_32(rhs.imm);
        }
      } else if index != RegisterX64::NOREG && rhs.scale != 1 {
        self.place(mod_rm(0b00, regop, 0b100));
        self.place(sib(rhs.scale, index.index(), 0b101));
        self.place_imm_32(rhs.imm);
      } else if (base.index() & 0x7) == 0b100 {
        // r12/sp-based addressing requires sib
        if !(rhs.scale == 1) {
          ulua_common::LUAU_DEBUGBREAK!();
        }
        if !(index == RegisterX64::NOREG) {
          ulua_common::LUAU_DEBUGBREAK!();
        }

        self.place(mod_rm(mod_, regop, 0b100));
        self.place(sib(rhs.scale, 0b100, base.index()));

        if rhs.imm != 0 {
          self.place_imm_8_or_32(rhs.imm);
        }
      } else if base == RegisterX64::RIP {
        self.place(mod_rm(0b00, regop, 0b101));

        // As a reminder: we do (getCodeSize() + 4) here to calculate the offset of the end of the current instruction we are placing.
        // Since we have already placed all of the instruction bytes for this instruction, we add +4 to account for the imm32 displacement.
        // Some instructions, however, are encoded such that an additional imm8 byte, or imm32 bytes, is placed after the ModRM byte, thus,
        // we need to account for that case here as well.
        self.place_imm_32(-((self.get_code_size() + 4 + extra_code_bytes as u32) as i32) + rhs.imm);
      } else if base != RegisterX64::NOREG {
        self.place(mod_rm(mod_, regop, base.index()));

        if mod_ != 0b00 {
          self.place_imm_8_or_32(rhs.imm);
        }
      } else {
        self.place(mod_rm(0b00, regop, 0b100));
        self.place(sib(1, 0b100, 0b101));
        self.place_imm_32(rhs.imm);
      }
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }
  }
}
