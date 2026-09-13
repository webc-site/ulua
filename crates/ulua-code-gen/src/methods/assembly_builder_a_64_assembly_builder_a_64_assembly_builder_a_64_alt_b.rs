use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn assembly_builder_a_64_assembly_builder_a_64_alt_b(&mut self) {
    // Avoid CODEGEN_ASSERT! here because ulua_common::assert_call_handler currently
    // expects raw pointers, while CODEGEN_ASSERT! passes &str values.
    debug_assert!(self.finalized);
    if !self.finalized {
      panic!("finalized assertion failed");
    }
  }
}
