use core::ffi::c_void;

use ulua_common::FInt::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};

use crate::{
  functions::create_shared_code_gen_context_code_gen_context_alt_c::create_shared_code_gen_context_usize_usize_allocation_callback_void,
  type_aliases::{
    allocation_callback::AllocationCallback,
    unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_shared_code_gen_context(
  allocation_callback: *mut AllocationCallback,
  allocation_callback_context: *mut c_void,
) -> UniqueSharedCodeGenContext {
  unsafe {
    create_shared_code_gen_context_usize_usize_allocation_callback_void(
      LuauCodeGenBlockSize.get() as usize,
      LuauCodeGenMaxTotalSize.get() as usize,
      allocation_callback,
      allocation_callback_context,
    )
  }
}
