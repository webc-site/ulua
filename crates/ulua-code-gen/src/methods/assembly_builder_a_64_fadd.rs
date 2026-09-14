use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fadd(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if dst.kind() == KindA64::D {
      debug_assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);
      self.place_r_3("fadd", dst, src1, src2, 0b1111_0011, 0b00_1010);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);
      self.place_r_3("fadd", dst, src1, src2, 0b1111_0001, 0b00_1010);
    } else {
      debug_assert!(
        dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q
      );
      self.place_vr("fadd", dst, src1, src2, 0b0_0111_0001, 0b11_0101);
    }
  }
}
