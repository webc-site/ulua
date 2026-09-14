use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn mov_register_a_64_i32(&mut self, dst: RegisterA64, src: i32) {
    if src >= 0 {
      self.movz(dst, (src & 0xffff) as u16, 0);
      if src > 0xffff {
        self.movk(dst, ((src >> 16) & 0xffff) as u16, 16);
      }
    } else {
      self.movn(dst, (!src & 0xffff) as u16, 0);
      if src < -0x10000 {
        self.movk(dst, ((src >> 16) & 0xffff) as u16, 16);
      }
    }
  }
}
