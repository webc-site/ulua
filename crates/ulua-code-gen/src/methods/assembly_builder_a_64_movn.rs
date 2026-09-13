use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn movn(&mut self, dst: RegisterA64, src: u16, shift: i32) {
    self.place_i16("movn", dst, src as i32, 0b00_100101, shift);
  }
}
