use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn place_imm_8(&mut self, imm: i32) {
    let imm8 = imm as i8;
    self.place(imm8 as u8);
  }
}
