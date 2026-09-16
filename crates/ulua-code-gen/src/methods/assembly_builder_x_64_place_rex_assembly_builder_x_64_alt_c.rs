use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::{rex_b::rex_b, rex_force::rex_force, rex_r::rex_r, rex_w_bit::REX_W_BIT, rex_x::rex_x},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn place_rex_register_x_64_operand_x_64(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    let mut code = REX_W_BIT!(lhs.size() == SizeX64::Qword) | rex_force(lhs);

    if rhs.cat == CategoryX64::Imm {
      code |= rex_b(lhs);
    } else {
      if !(rhs.cat == CategoryX64::Reg || rhs.cat == CategoryX64::Mem) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      code |=
        rex_r(lhs) | rex_x(rhs.index) | rex_b(rhs.base) | rex_force(lhs) | rex_force(rhs.base);
    }

    if code != 0 {
      self.place(code | 0x40);
    }
  }
}
