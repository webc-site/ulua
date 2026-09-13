use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn not_(&mut self, op: OperandX64) {
    // C++ `placeUnaryModRegMem("not", op, 0xf6, 0xf7, 2)` — must emit the
    // size-select opcode (0xf6/0xf7) + REX, not a bare ModRM byte.
    self.place_unary_mod_reg_mem("not", op, 0xf6, 0xf7, 2);
  }
}
