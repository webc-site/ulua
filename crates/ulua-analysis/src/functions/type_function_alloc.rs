use core::{
  ffi::c_void,
  ptr::{copy_nonoverlapping, null_mut},
};
use std::alloc::{Layout, alloc, dealloc};
/// 分配器契约与 VM `LuaAlloc` 一致：块用 `*mut u8` 表达字节语义，ud 保留 c_void。
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe extern "C-unwind" fn type_function_alloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  osize: usize,
  nsize: usize,
) -> *mut u8 {
  unsafe {
    if nsize == 0 {
      if !ptr.is_null() {
        dealloc(ptr, Layout::from_size_align_unchecked(osize, 8));
      }
      null_mut()
    } else if osize == 0 {
      alloc(Layout::from_size_align_unchecked(nsize, 8))
    } else {
      let data = alloc(Layout::from_size_align_unchecked(nsize, 8));

      if !data.is_null() && !ptr.is_null() {
        let copy_size = if nsize < osize { nsize } else { osize };
        copy_nonoverlapping(ptr, data, copy_size);

        dealloc(ptr, Layout::from_size_align_unchecked(osize, 8));
      }

      data
    }
  }
}
