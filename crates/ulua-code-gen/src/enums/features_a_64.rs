//! Source: `CodeGen/include/Luau/AssemblyBuilderA64.h:20`

crate::flag_enum! {
  pub enum FeaturesA64: u32 {
    FeatureJscvt = 1 << 0,
    FeatureAdvSimd = 1 << 1,
  }
  aliases {
    FEATURE_JSCVT = FeatureJscvt,
    FEATURE_ADV_SIMD = FeatureAdvSimd,
  }
}
