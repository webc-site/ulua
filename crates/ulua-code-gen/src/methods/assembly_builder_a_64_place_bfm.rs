use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_bfm(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: i32,
    op: u8,
    immr: i32,
    imms: i32,
  ) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_i32(name, dst, src1, src2);
    }

    // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
    // in assert_call_handler arguments elsewhere in the codebase.
    debug_assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    debug_assert!(dst.kind() == src1.kind());

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };
    let n = if dst.kind() == KindA64::X { 1 << 22 } else { 0 };

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((imms as u32) << 10)
        | ((immr as u32) << 16)
        | n
        | ((op as u32) << 23)
        | sf,
    );
    self.commit();
  }
}
