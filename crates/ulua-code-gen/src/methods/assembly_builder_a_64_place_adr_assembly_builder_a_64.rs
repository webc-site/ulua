use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_adr_c_char_register_a_64_u8(&mut self, name: &str, dst: RegisterA64, op: u8) {
    if self.log_text {
      self.log_c_char_register_a_64(name, dst);
    }

    // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
    // in assert_call_handler arguments elsewhere in the codebase.
    debug_assert!(dst.kind() == KindA64::X);

    self.place((dst.index() as u32) | ((op as u32) << 24));
    self.commit();
  }
}
