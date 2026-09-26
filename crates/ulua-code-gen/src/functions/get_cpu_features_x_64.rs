use crate::enums::features_x_64::FeaturesX64;

/// CPUID leaf 1 的 ECX 特性位：FMA3 = bit12、AVX1 = bit28
const K_CPUID_ECX_FMA3: i32 = 1 << 12;
const K_CPUID_ECX_AVX: i32 = 1 << 28;

pub fn get_cpu_features_x_64() -> u32 {
  let mut result: u32 = 0;

  #[cfg(target_arch = "x86_64")]
  let cpuinfo = if crate::macros::codegen_target_x_64::CODEGEN_TARGET_X64 {
    use core::arch::x86_64::__cpuid;
    let r = __cpuid(1);
    [r.eax as i32, r.ebx as i32, r.ecx as i32, r.edx as i32]
  } else {
    [0, 0, 0, 0]
  };

  #[cfg(target_arch = "x86")]
  let cpuinfo = if crate::macros::codegen_target_x_64::CODEGEN_TARGET_X64 {
    use core::arch::x86::__cpuid;
    let r = __cpuid(1);
    [r.eax as i32, r.ebx as i32, r.ecx as i32, r.edx as i32]
  } else {
    [0, 0, 0, 0]
  };

  #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
  let cpuinfo: [i32; 4] = [0, 0, 0, 0];

  let feature_fma3 = FeaturesX64::FeatureFma3 as u32;
  let feature_avx = FeaturesX64::FeatureAvx as u32;

  if (cpuinfo[2] & K_CPUID_ECX_FMA3) != 0 {
    result |= feature_fma3;
  }

  if (cpuinfo[2] & K_CPUID_ECX_AVX) != 0 {
    result |= feature_avx;
  }

  result
}
