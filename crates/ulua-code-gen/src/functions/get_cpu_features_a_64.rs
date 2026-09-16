use core::ffi::{c_char, c_void};
pub fn get_cpu_features_a_64() -> u32 {
  #[cfg(target_os = "macos")]
  {
    use core::{mem, ptr};

    use crate::enums::features_a_64::FeaturesA64;

    unsafe extern "C" {
      fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut usize,
        newp: *mut c_void,
        newlen: usize,
      ) -> i32;
    }

    let mut result: u32 = 0;
    unsafe {
      let mut jscvt: i32 = 0;
      let mut jscvt_len = mem::size_of_val(&jscvt);
      if sysctlbyname(
        c"hw.optional.arm.FEAT_JSCVT".as_ptr(),
        &mut jscvt as *mut _ as *mut c_void,
        &mut jscvt_len,
        ptr::null_mut(),
        0,
      ) == 0
        && jscvt == 1
      {
        result |= FeaturesA64::FeatureJscvt as u32;
      }

      let mut adv_simd: i32 = 0;
      let mut adv_simd_len = mem::size_of_val(&adv_simd);
      if sysctlbyname(
        c"hw.optional.arm.AdvSIMD".as_ptr(),
        &mut adv_simd as *mut _ as *mut c_void,
        &mut adv_simd_len,
        ptr::null_mut(),
        0,
      ) == 0
        && adv_simd == 1
      {
        result |= FeaturesA64::FeatureAdvSimd as u32;
      }
    }
    result
  }

  #[cfg(not(target_os = "macos"))]
  {
    0
  }
}
