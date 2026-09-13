use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn rol(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("rol", lhs, rhs, 0);
  }
}
