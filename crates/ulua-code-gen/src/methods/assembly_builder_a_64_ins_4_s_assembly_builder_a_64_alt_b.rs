use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn ins_4_s_register_a_64_u8_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    dst_index: u8,
    src: RegisterA64,
    src_index: u8,
  ) {
    debug_assert!(dst.kind() == KindA64::Q && src.kind() == KindA64::Q);
    debug_assert!(dst_index < 4);
    debug_assert!(src_index < 4);

    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.s[{}],v{}.s[{}]\n",
        "ins",
        dst.index(),
        dst_index,
        src.index(),
        src_index
      ));
    }

    let op: u32 = 0b01_1011_1000_0001_0000_0001;

    self.place(
      (dst.index() as u32)
        | ((src.index() as u32) << 5)
        | (op << 10)
        | ((dst_index as u32) << 19)
        | ((src_index as u32) << 13),
    );
    self.commit();
  }
}
