use crate::{
  enums::{category_x_64::CategoryX64, condition_x_64::ConditionX64, size_x_64::SizeX64},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn setcc(&mut self, cond: ConditionX64, op: OperandX64) {
    let size = if op.cat == CategoryX64::Reg {
      op.base.size()
    } else {
      op.mem_size
    };

    if !(size == SizeX64::Byte) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      static SETCC_TEXT: [&str; 26] = [
        "seto", "setno", "setc", "setnc", "setb", "setbe", "seta", "setae", "sete", "setl",
        "setle", "setg", "setge", "setnb", "setnbe", "setna", "setnae", "setne", "setnl", "setnle",
        "setng", "setnge", "setz", "setnz", "setp", "setnp",
      ];

      let cond_idx = cond as usize;
      self.log_c_char_operand_x_64(SETCC_TEXT[cond_idx], op);
    }

    self.place_rex_operand_x_64(op);
    self.place(0x0f);

    static CODE_FOR_CONDITION: [u8; 26] = [
      0x00, 0x01, 0x02, 0x03, 0x02, 0x06, 0x07, 0x03, 0x04, 0x0c, 0x0e, 0x0f, 0x0d, 0x03, 0x07,
      0x06, 0x02, 0x05, 0x0d, 0x0f, 0x0e, 0x0c, 0x04, 0x05, 0x0a, 0x0b,
    ];

    self.place(0x90 | CODE_FOR_CONDITION[cond as usize]);
    self.place_mod_reg_mem(op, 0, 0);
    self.commit();
  }
}
