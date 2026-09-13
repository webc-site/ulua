use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn inc(&mut self, op: OperandX64) {
    // C++ `placeUnaryModRegMem("inc", op, 0xfe, 0xff, 0)` — must emit the
    // size-select opcode (0xfe/0xff) + REX, not a bare ModRM byte.
    self.place_unary_mod_reg_mem("inc", op, 0xfe, 0xff, 0);
  }
}
