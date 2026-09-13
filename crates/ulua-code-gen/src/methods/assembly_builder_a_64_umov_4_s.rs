use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn umov_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
    CODEGEN_ASSERT!(dst.kind() == KindA64::W);
    CODEGEN_ASSERT!(src.kind() == KindA64::Q);
    CODEGEN_ASSERT!(index < 4);

    if self.log_text {
      self.log_append(format_args!(
        " {:<12}w{},v{}.s[{}]\n",
        "umov",
        dst.index(),
        src.index(),
        index
      ));
    }

    let op: u32 = 0b0000_1110_0000_0100_0011_1100_0000_0000;

    self.place(dst.index() as u32 | (src.index() as u32) << 5 | op | (index as u32) << 19);

    self.commit();
  }
}
