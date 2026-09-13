#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FeaturesX64 {
  FeatureFma3 = 1 << 0,
  FeatureAvx = 1 << 1,
}

impl FeaturesX64 {
  pub const FEATURE_FMA3: FeaturesX64 = FeaturesX64::FeatureFma3;
  pub const FEATURE_AVX: FeaturesX64 = FeaturesX64::FeatureAvx;
}
