use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_i12(&mut self, name: &str, dst: RegisterA64, src1: RegisterA64, src2: i32, op: u8) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_i32(name, dst, src1, src2);
    }

    // Avoid CODEGEN_ASSERT! here: it currently expands to ulua_common::assert_call_handler
    // which expects *const i8 parameters, but the macro supplies &str.
    // Keep the same logical checks using plain Rust asserts instead.

    assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X || dst == RegisterA64::SP);
    assert!(
      dst.kind() == src1.kind()
        || (dst.kind() == KindA64::X && src1 == RegisterA64::SP)
        || (dst == RegisterA64::SP && src1.kind() == KindA64::X)
    );
    assert!((0..(1 << 12)).contains(&src2));

    let sf = if dst.kind() != KindA64::W {
      0x80000000
    } else {
      0
    };

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((src2 as u32) << 10)
        | ((op as u32) << 24)
        | sf,
    );
    self.commit();
  }
}
