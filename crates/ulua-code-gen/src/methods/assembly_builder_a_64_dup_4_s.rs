use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn dup_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
    if dst.kind() == KindA64::S {
      CODEGEN_ASSERT!(src.kind() == KindA64::Q);
      CODEGEN_ASSERT!(index < 4);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}s{},v{}.s[{}]\n",
          "dup",
          dst.index(),
          src.index(),
          index
        ));
      }

      let op: u32 = 0b01_0111_1000_0001_0000_0001;
      self.place(dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19);
    } else {
      CODEGEN_ASSERT!(src.kind() == KindA64::Q);
      CODEGEN_ASSERT!(index < 4);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}v{}.4s,v{}.s[{}]\n",
          "dup",
          dst.index(),
          src.index(),
          index
        ));
      }

      let op: u32 = 0b01_0011_1000_0001_0000_0001;
      self.place(dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19);
    }

    self.commit();
  }
}
