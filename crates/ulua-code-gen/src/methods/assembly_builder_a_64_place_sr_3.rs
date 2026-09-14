use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_sr_3(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    shift: i32,
    n: i32,
  ) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_register_a_64_i32(name, dst, src1, src2, shift);
    }

    // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
    // in assert_call_handler arguments elsewhere in the codebase.
    debug_assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    debug_assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());
    debug_assert!((-63..=63).contains(&shift));

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    let shift_abs = if shift < 0 { -shift } else { shift };

    self.place(
      dst.index() as u32
        | (src1.index() as u32) << 5
        | (shift_abs as u32) << 10
        | (src2.index() as u32) << 16
        | ((n as u32) << 21)
        | ((shift < 0) as u32) << 22
        | (op as u32) << 24
        | sf,
    );
    self.commit();
  }
}
