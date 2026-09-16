use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_br(&mut self, name: &str, src: RegisterA64, op: u32) {
    if self.log_text {
      self.log_c_char_register_a_64(name, src);
    }

    // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
    // in assert_call_handler arguments elsewhere in the codebase.
    debug_assert!(src.kind() == KindA64::X);

    self.place((src.index() as u32) << 5 | (op << 10));
    self.commit();
  }
}
