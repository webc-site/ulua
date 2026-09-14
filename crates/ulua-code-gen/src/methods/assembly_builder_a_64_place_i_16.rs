use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_i16(&mut self, name: &str, dst: RegisterA64, src: i32, op: u8, shift: i32) {
    if self.log_text {
      self.log_c_char_register_a_64_i32_i32(name, dst, src, shift);
    }

    // Avoid CODEGEN_ASSERT! here: it expands to ulua_common::assert_call_handler
    // which expects *const i8 parameters, but the macro supplies &str.
    assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    assert!((0..=0xffff).contains(&src));
    assert!(shift == 0 || shift == 16 || shift == 32 || shift == 48);

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    self.place(
      (dst.index() as u32)
        | ((src as u32) << 5)
        | (((shift >> 4) as u32) << 21)
        | ((op as u32) << 23)
        | sf,
    );
    self.commit();
  }
}
