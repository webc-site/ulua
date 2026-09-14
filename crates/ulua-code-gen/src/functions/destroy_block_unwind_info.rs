use core::ffi::{c_char, c_void};

#[cfg(target_os = "windows")]
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::{
  functions::visit_fde_entries::visit_fde_entries,
  macros::{codegen_target_a_64::CODEGEN_TARGET_A64, codegen_target_x_64::CODEGEN_TARGET_X64},
};

#[cfg(target_os = "windows")]
unsafe extern "system" {
  fn RtlDeleteFunctionTable(function_table: *mut core::ffi::c_void) -> i32;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
unsafe extern "C" {
  fn __deregister_frame(begin: *const c_void);
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn destroy_block_unwind_info(
  _context: *mut c_void,
  unwind_data: *mut c_void,
) {
  #[cfg(target_os = "windows")]
  {
    if CODEGEN_TARGET_X64 {
      let result = unsafe { RtlDeleteFunctionTable(unwind_data) };
      if result == 0 {
        CODEGEN_ASSERT!(false);
      }
    }
  }

  #[cfg(any(target_os = "linux", target_os = "macos"))]
  {
    if CODEGEN_TARGET_X64 || CODEGEN_TARGET_A64 {
      unsafe {
        visit_fde_entries(unwind_data as *mut c_char, __deregister_frame);
      }
    }
  }
}
