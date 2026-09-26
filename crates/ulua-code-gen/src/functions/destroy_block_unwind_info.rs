#[cfg(any(target_os = "linux", target_os = "macos"))]
use core::ffi::c_char;
use core::ffi::c_void;

#[cfg(target_os = "windows")]
use crate::macros::codegen_assert::CODEGEN_ASSERT;
// wasm32 等无帧表注销机制的目标上本函数整体为空操作，两个宏均不参与编译。
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use crate::macros::codegen_target::CODEGEN_TARGET_X64;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::{
  functions::visit_fde_entries::visit_fde_entries, macros::codegen_target::CODEGEN_TARGET_A64,
};

#[cfg(target_os = "windows")]
unsafe extern "system" {
  fn RtlDeleteFunctionTable(function_table: *mut c_void) -> i32;
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
      // Safety: 契约保证 unwind_data 指向本 codegen 此前以 RtlAddFunctionTable 登记、尚未注销的
      // RUNTIME_FUNCTION 表（与 C++ destroyBlockUnwindInfo 的注销路径同构），指针非空、对齐且
      // 在本调用点仍存活；返回 0 表示注销失败，按上方 CODEGEN_ASSERT 记录，不解引用返回值。
      let result = unsafe { RtlDeleteFunctionTable(unwind_data) };
      if result == 0 {
        CODEGEN_ASSERT!(false);
      }
    }
  }

  #[cfg(any(target_os = "linux", target_os = "macos"))]
  {
    if CODEGEN_TARGET_X64 || CODEGEN_TARGET_A64 {
      // Safety: 契约保证 unwind_data 为本 codegen 分配、与持有者同寿的块 unwind 信息指针；
      // visit_fde_entries 沿其枚举 FDE，并对每条以 C ABI 调用 __deregister_frame（该函数接受
      // 此前由 __register_frame 登记过的合法帧表指针），与 C++ 注销路径同构。
      unsafe {
        visit_fde_entries(unwind_data as *mut c_char, __deregister_frame);
      }
    }
  }

  #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
  {
    // wasm32 等目标没有运行期帧表登记机制（codegen 不在这些目标上发射可注销的
    // unwind 信息），本函数在其余目标上按定义即为空操作。
    let _ = unwind_data;
  }
}
