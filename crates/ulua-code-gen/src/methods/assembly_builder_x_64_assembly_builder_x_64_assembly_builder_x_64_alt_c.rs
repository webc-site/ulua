use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn assembly_builder_x_64_assembly_builder_x_64_assembly_builder_x_64_alt_c(&mut self) {
    // Avoid CODEGEN_ASSERT! here because ulua_common::assert_call_handler currently
    // expects raw pointers, while CODEGEN_ASSERT! passes &str values.
    debug_assert!(self.finalized);
    if !self.finalized {
      panic!("finalized assertion failed");
    }
  }
}
