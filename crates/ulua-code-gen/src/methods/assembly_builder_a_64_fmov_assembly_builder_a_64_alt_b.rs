use crate::{
  enums::kind_a_64::KindA64,
  functions::get_fmov_imm_fp_64::get_fmov_imm_fp_64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fmov_register_a_64_f64(&mut self, dst: RegisterA64, src: f64) {
    let dst_kind = dst.kind();
    debug_assert!(dst_kind == KindA64::D || dst_kind == KindA64::Q);

    let imm = get_fmov_imm_fp_64(src);
    debug_assert!((0..=256).contains(&imm));

    // fmov can't encode 0, but movi can; movi is otherwise not useful for fp immediates because it encodes repeating patterns
    if dst_kind == KindA64::D {
      if imm == 256 {
        self.place_fmov("movi", dst, src, 0b001_0111_1000_0000_0111_0010_0000);
      } else {
        self.place_fmov(
          "fmov",
          dst,
          src,
          0b000_1111_0011_0000_0000_1000_0000 | ((imm as u32) << 8),
        );
      }
    } else {
      if imm == 256 {
        self.place_fmov("movi.4s", dst, src, 0b010_0111_1000_0000_0000_0010_0000);
      } else {
        self.place_fmov(
          "fmov.4s",
          dst,
          src,
          0b010_0111_1000_0000_0111_1010_0000 | ((((imm as u32) >> 5) << 11) | (imm as u32 & 31)),
        );
      }
    }
  }
}
