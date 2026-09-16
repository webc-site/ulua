use crate::{
  enums::kind_a_64::KindA64,
  functions::{countlz_bit_utils::countlz, countrz_bit_utils::countrz},
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_bm(&mut self, name: &str, dst: RegisterA64, src1: RegisterA64, src2: u32, op: u8) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_i32(name, dst, src1, src2 as i32);
    }

    assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    assert!(dst.kind() == src1.kind());
    assert!(Self::is_mask_supported(self, src2));

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    let lz = countlz(src2);
    let rz = countrz(src2);

    let imms = 31 - lz - rz;
    let immr = (32 - rz) & 31;

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((imms as u32) << 10)
        | ((immr as u32) << 16)
        | ((op as u32) << 23)
        | sf,
    );
    self.commit();
  }
}
