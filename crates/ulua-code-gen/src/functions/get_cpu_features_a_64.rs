pub fn get_cpu_features_a_64() -> u32 {
  #[cfg(target_os = "macos")]
  {
    use core::{
      ffi::{c_char, c_void},
      mem, ptr,
    };

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
    // 总契约（下文两处窄块统一援引，简记「依契约」）：
    // Safety: sysctlbyname 为 C ABI 只读查询——name 取 `&[u8]` NUL 结尾字节串常量的首指针
    // （review.md §10 收口点 `.as_ptr().cast()`）；
    // oldp 指向本函数栈上存活 i32、oldlenp 传入其 size_of；newp=null 配 newlen=0 是读语义
    // 的合法 sysctl 组合，实参无对齐或别名风险。
    let mut jscvt: i32 = 0;
    let mut jscvt_len = mem::size_of_val(&jscvt);
    // Safety: 依契约；返回值仅与本函数栈上读数比较。
    let jscvt_ok = unsafe {
      sysctlbyname(
        c"hw.optional.arm.FEAT_JSCVT".as_ptr(),
        &mut jscvt as *mut _ as *mut c_void,
        &mut jscvt_len,
        ptr::null_mut(),
        0,
      )
    };
    if jscvt_ok == 0 && jscvt == 1 {
      result |= FeaturesA64::FeatureJscvt;
    }

    let mut adv_simd: i32 = 0;
    let mut adv_simd_len = mem::size_of_val(&adv_simd);
    // Safety: 依契约；返回值仅与本函数栈上读数比较。
    let adv_simd_ok = unsafe {
      sysctlbyname(
        c"hw.optional.arm.AdvSIMD".as_ptr(),
        &mut adv_simd as *mut _ as *mut c_void,
        &mut adv_simd_len,
        ptr::null_mut(),
        0,
      )
    };
    if adv_simd_ok == 0 && adv_simd == 1 {
      result |= FeaturesA64::FeatureAdvSimd;
    }
    result
  }

  #[cfg(not(target_os = "macos"))]
  {
    0
  }
}
