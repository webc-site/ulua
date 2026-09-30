use core::{ffi::c_void, ptr};

use crate::records::info_code_allocator_test::Info;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn create_block_unwind_info_code_allocator_test(
  context: *mut c_void,
  block: *mut u8,
  _block_size: usize,
  begin_offset: &mut usize,
) -> *mut c_void {
  // Safety: CodeAllocator C ABI 回调契约：context 指向测试传入的 Info（非空、对齐、比回调帧长寿），block 为刚分配 block_size 字节的可写区（本 fixture  unwind 长 8 与断言一致）；&mut 重借用仅帧内改记账字段，copy 8 字节进界内块；返回 Box::into_raw 堆指针，所有权按契约移交给 destroy 回调（单次回收）。
  unsafe {
    let info = &mut *(context.cast::<Info>());

    assert_eq!(info.unwind.len(), 8);
    ptr::copy_nonoverlapping(info.unwind.as_ptr(), block, info.unwind.len());
    *begin_offset = 8;
    info.block = block as usize;

    Box::into_raw(Box::new(7_i32)).cast()
  }
}
