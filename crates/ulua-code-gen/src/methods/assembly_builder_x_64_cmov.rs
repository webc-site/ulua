use crate::{
  enums::{category_x_64::CategoryX64, condition_x_64::ConditionX64, size_x_64::SizeX64},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn cmov(&mut self, cond: ConditionX64, lhs: RegisterX64, rhs: OperandX64) {
    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };

    if !(size != SizeX64::Byte && size == lhs.size()) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      let cond_idx = cond as usize;
      let cmov_text = [
        "cmovo", "cmovno", "cmovc", "cmovnc", "cmovb", "cmovbe", "cmova", "cmovae", "cmove",
        "cmovl", "cmovle", "cmovg", "cmovge", "cmovnb", "cmovnbe", "cmovna", "cmovnae", "cmovne",
        "cmovnl", "cmovnle", "cmovng", "cmovnge", "cmovz", "cmovnz", "cmovp", "cmovnp",
      ];
      self.log_c_char_operand_x_64_operand_x_64(cmov_text[cond_idx], OperandX64::reg(lhs), rhs);
    }

    self.place_rex_register_x_64_operand_x_64(lhs, rhs);
    self.place(0x0f);

    let code_for_condition = [
      0x0, 0x1, 0x2, 0x3, 0x2, 0x6, 0x7, 0x3, 0x4, 0xc, 0xe, 0xf, 0xd, 0x3, 0x7, 0x6, 0x2, 0x5,
      0xd, 0xf, 0xe, 0xc, 0x4, 0x5, 0xa, 0xb,
    ];
    self.place(0x40 | code_for_condition[cond as usize]);

    self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
    self.commit();
  }
}
