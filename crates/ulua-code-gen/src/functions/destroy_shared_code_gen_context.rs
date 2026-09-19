use alloc::alloc::dealloc;
use core::{alloc::Layout, ptr::drop_in_place};

use crate::records::shared_code_gen_context::SharedCodeGenContext;

/// # Safety
///
/// This function is native-only and performs a manual memory deallocation.
/// The caller must ensure that `code_gen_context` was created by a matching
/// allocation function and that it is not used after this call.
#[unsafe(export_name = "ulua_destroy_shared_code_gen_context")]
pub unsafe extern "C-unwind" fn destroy_shared_code_gen_context(
  code_gen_context: *const SharedCodeGenContext,
) {
  if !code_gen_context.is_null() {
    let ptr = code_gen_context as *mut SharedCodeGenContext;
    unsafe {
      drop_in_place(ptr);
      dealloc(ptr as *mut u8, Layout::new::<SharedCodeGenContext>());
    }
  }
}
