#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CategoryX64 {
  Reg,
  Mem,
  Imm,
}

// Source: `CodeGen/include/Luau/AssemblyBuilderX64.h:24`

crate::flag_enum! {
  pub enum FeaturesX64: u32 {
    FeatureFma3 = 1 << 0,
    FeatureAvx = 1 << 1,
  }
}
