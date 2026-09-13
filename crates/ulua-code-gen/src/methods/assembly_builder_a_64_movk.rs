use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn movk(&mut self, dst: RegisterA64, src: u16, shift: i32) {
    self.place_i16("movk", dst, src as i32, 0b11100101, shift);
  }
}
