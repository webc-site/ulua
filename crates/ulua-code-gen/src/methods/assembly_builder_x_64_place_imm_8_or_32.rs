use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn place_imm_8_or_32(&mut self, imm: i32) {
    let imm8 = imm as i8 as i32;
    if imm8 == imm {
      self.place_imm_8(imm8);
    } else {
      self.place_imm_32(imm);
    }
  }
}
