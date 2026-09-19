#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FeaturesA64 {
  FeatureJscvt = 1 << 0,
  FeatureAdvSimd = 1 << 1,
}

impl FeaturesA64 {
  pub const FEATURE_JSCVT: FeaturesA64 = FeaturesA64::FeatureJscvt;
  pub const FEATURE_ADV_SIMD: FeaturesA64 = FeaturesA64::FeatureAdvSimd;
}
