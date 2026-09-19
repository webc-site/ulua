use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::{rex_b::rex_b, rex_force::rex_force, rex_w_bit::REX_W_BIT, rex_x::rex_x},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn place_rex_operand_x_64(&mut self, op: OperandX64) {
    let mut code: u8 = 0;

    if op.cat == CategoryX64::Reg {
      code = REX_W_BIT!(op.base.size() == SizeX64::Qword) | rex_b(op.base) | rex_force(op.base);
    } else if op.cat == CategoryX64::Mem {
      code = REX_W_BIT!(op.mem_size == SizeX64::Qword) | rex_x(op.index) | rex_b(op.base);
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if code != 0 {
      self.place(code | 0x40);
    }
  }
}
