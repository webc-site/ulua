use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_e_r(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    shift: i32,
  ) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_register_a_64_i32(name, dst, src1, src2, shift);
    }

    // CODEGEN_ASSERT! is currently incompatible with ulua_common::assert_call_handler
    // due to mismatched argument types (expects *const i8, macro supplies &str).
    // Mirror the checks using plain Rust asserts instead.
    assert!(dst.kind() == KindA64::X && src1.kind() == KindA64::X);
    assert!(src2.kind() == KindA64::W);
    assert!((0..=4).contains(&shift));

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    let option = 0b010; // UXTW

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((shift as u32) << 10)
        | ((option as u32) << 13)
        | ((src2.index() as u32) << 16)
        | (1 << 21)
        | ((op as u32) << 24)
        | sf,
    );
    self.commit();
  }
}
