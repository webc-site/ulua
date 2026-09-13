use crate::{
  enums::kind_a_64::KindA64,
  functions::get_fmov_imm_fp_32::get_fmov_imm_fp_32,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn fmov_register_a_64_f32(&mut self, dst: RegisterA64, src: f32) {
    debug_assert!(dst.kind() == KindA64::S || dst.kind() == KindA64::Q);

    let imm = get_fmov_imm_fp_32(src);
    debug_assert!((0..=256).contains(&imm));

    // fmov can't encode 0, but movi can; movi is otherwise not useful for fp immediates because it encodes repeating patterns
    if dst.kind() == KindA64::S {
      if imm == 256 {
        self.place_fmov("movi", dst, src as f64, 0b001_0111_1000_0000_0111_0010_0000);
      } else {
        self.place_fmov(
          "fmov",
          dst,
          src as f64,
          0b000_1111_0001_0000_0000_1000_0000 | ((imm as u32) << 8),
        );
      }
    } else {
      if imm == 256 {
        self.place_fmov(
          "movi.4s",
          dst,
          src as f64,
          0b010_0111_1000_0000_0000_0010_0000,
        );
      } else {
        self.place_fmov(
          "fmov.4s",
          dst,
          src as f64,
          0b010_0111_1000_0000_0111_1010_0000 | (((imm as u32) >> 5) << 11) | (imm as u32 & 31),
        );
      }
    }
  }
}
