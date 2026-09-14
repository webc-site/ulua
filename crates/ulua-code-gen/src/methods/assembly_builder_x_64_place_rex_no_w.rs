use crate::{
  enums::category_x_64::CategoryX64,
  macros::{rex_b::rex_b, rex_x::rex_x},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn place_rex_no_w(&mut self, op: OperandX64) {
    let mut code: u8 = 0;

    if op.cat == CategoryX64::Reg {
      code = rex_b(op.base);
    } else if op.cat == CategoryX64::Mem {
      code = rex_x(op.index) | rex_b(op.base);
    } else {
      // Avoid CODEGEN_ASSERT! due to assert_call_handler signature mismatch in this crate.
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if code != 0 {
      self.place(code | 0x40);
    }
  }
}
