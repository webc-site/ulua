use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::{
  alloc::Layout,
  ffi::c_void,
  ptr::{NonNull, drop_in_place},
};

use crate::{
  records::shared_code_gen_context::SharedCodeGenContext,
  type_aliases::{
    allocation_callback::AllocationCallback,
    unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_shared_code_gen_context_usize_usize_allocation_callback_void(
  block_size: usize,
  max_total_size: usize,
  allocation_callback: *mut AllocationCallback,
  allocation_callback_context: *mut c_void,
) -> UniqueSharedCodeGenContext {
  unsafe {
    let layout = Layout::new::<SharedCodeGenContext>();
    let ptr = alloc(layout) as *mut SharedCodeGenContext;
    if ptr.is_null() {
      handle_alloc_error(layout);
    }

    (*ptr).shared_code_gen_context_shared_code_gen_context(
      block_size,
      max_total_size,
      allocation_callback,
      allocation_callback_context,
    );

    if !(*ptr).base.init_header_functions() {
      drop_in_place(ptr);
      dealloc(ptr as *mut u8, layout);
      return NonNull::dangling();
    }

    NonNull::new_unchecked(ptr)
  }
}
