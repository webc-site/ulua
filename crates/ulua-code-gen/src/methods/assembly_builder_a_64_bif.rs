use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn bif(&mut self, dst: RegisterA64, src: RegisterA64, mask: RegisterA64) {
    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.16b,v{}.16b,v{}.16b\n",
        "bif",
        dst.index(),
        src.index(),
        mask.index()
      ));
    }

    let op: u32 = 0b0110_1110_1110_0000_0001_1100_0000_0000;

    self.place(
      (dst.index() as u32) | ((src.index() as u32) << 5) | ((mask.index() as u32) << 16) | op,
    );

    self.commit();
  }
}
