use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn ins_4_s_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src: RegisterA64,
    index: u8,
  ) {
    debug_assert!(dst.kind() == KindA64::Q && src.kind() == KindA64::W);
    debug_assert!(index < 4);

    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.s[{}],w{}\n",
        "ins",
        dst.index(),
        index,
        src.index()
      ));
    }

    let op: u32 = 0b01_0011_1000_0001_0000_0111;

    self.place(dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19);
    self.commit();
  }
}
