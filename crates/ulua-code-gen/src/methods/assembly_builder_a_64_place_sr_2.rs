use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_sr_2(&mut self, name: &str, dst: RegisterA64, src: RegisterA64, op: u8, op2: u8) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64(name, dst, src);
    }

    // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
    // in assert_call_handler arguments elsewhere in the codebase.
    debug_assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    debug_assert!(dst.kind() == src.kind());

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    self.place(
      dst.index() as u32
        | (0x1f << 5)
        | (src.index() as u32) << 16
        | (op2 as u32) << 21
        | (op as u32) << 24
        | sf,
    );
    self.commit();
  }
}
