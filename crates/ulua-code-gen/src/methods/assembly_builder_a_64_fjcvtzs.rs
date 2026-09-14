use crate::{
  enums::{features_a_64::FeaturesA64, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fjcvtzs(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == KindA64::W);
    debug_assert!(src.kind() == KindA64::D);
    debug_assert!(self.features & FeaturesA64::FeatureJscvt as u32 != 0);

    self.place_r_1("fjcvtzs", dst, src, 0b00_0111_1001_1111_1000_0000);
  }
}
