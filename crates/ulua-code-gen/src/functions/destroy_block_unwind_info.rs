use core::ffi::c_void;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use core::slice::from_raw_parts;

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
/// 依 `CodeAllocator::destroy_block_unwind_info` 槽契约：`unwind_data` 指向此前
/// `create_block_unwind_info` 登记、尚未解除映射的存活代码块基址，且该块至少
/// `unwind_block_size` 字节可读；`context` 为本 crate 接线的 opaque unwind 构建器指针
/// （本实现不消费）。
pub unsafe extern "C-unwind" fn destroy_block_unwind_info(
  _context: *mut c_void,
  unwind_data: *mut c_void,
  unwind_block_size: usize,
) {
  #[cfg(target_os = "windows")]
  {
    // Windows 面注销整表，无需长度上界；linux/macos 面用其构造切片。
    let _ = unwind_block_size;
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
      // Safety: 函数头契约保证 unwind_data 指向至少 unwind_block_size 字节可读的存活块
      // （CodeAllocator::destroy 在 free_pages 之前逐句柄回调，页映射仍有效）；切片仅读。
      let block = unsafe { from_raw_parts(unwind_data.cast::<u8>(), unwind_block_size) };
      // FDE 遍历与 __deregister_frame（C ABI）回调收口在 visit_fde_entries 的
      // 切片游标模型内，与 C++ 注销路径同构。
      visit_fde_entries(block, __deregister_frame);
    }
  }

  #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
  {
    // wasm32 等目标没有运行期帧表登记机制（codegen 不在这些目标上发射可注销的
    // unwind 信息），本函数在其余目标上按定义即为空操作。
    let _ = unwind_data;
    let _ = unwind_block_size;
  }
}
