use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn fcmgt_4s(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.4s,v{}.4s,v{}.4s\n",
        "fcmgt",
        dst.index(),
        src1.index(),
        src2.index()
      ));
    }

    //                Q U      ESz Rm    Opcode Rn    Rd
    let op: u32 = 0b0110_1110_1010_0000_1110_0100_0000_0000;

    self.place(dst.index() as u32 | (src1.index() as u32) << 5 | (src2.index() as u32) << 16 | op);

    self.commit();
  }
}
