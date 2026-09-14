use crate::{
  macros::codegen_assert::CODEGEN_ASSERT, records::assembly_builder_a_64::AssemblyBuilderA64,
};

impl AssemblyBuilderA64 {
  pub fn commit(&mut self) {
    CODEGEN_ASSERT!(self.code_pos <= self.code_end);

    if self.code_end == self.code_pos {
      self.extend();
    }
  }
}
