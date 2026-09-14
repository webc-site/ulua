use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_r_1(&mut self, name: &str, dst: RegisterA64, src: RegisterA64, op: u32) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64(name, dst, src);
    }

    let sf: u32 = if dst.kind() == KindA64::X || src.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    self.place(dst.index() as u32 | ((src.index() as u32) << 5) | (op << 10) | sf);
    self.commit();
  }
}
