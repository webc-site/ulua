use alloc::string::String;
use core::{
  ffi::{c_char, c_void},
  ptr::copy_nonoverlapping,
  slice::from_raw_parts_mut,
};

pub use ulua_require::enums::luarequire_write_result::LuarequireWriteResult as luarequire_WriteResult;

/// # Safety
///
/// `contents` must either be null or point to a valid `String`.
/// If `buffer` is non-null, it must be valid for writes of `null_terminated_size` bytes.
/// If `size_out` is non-null, it must be valid for writing a `usize`.
pub unsafe fn write(
  contents: *const c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  if contents.is_null() {
    return luarequire_WriteResult::WRITE_FAILURE;
  }

  let s = contents as *const String;
  let contents = unsafe { &*s };

  let null_terminated_size = contents.len() + 1;

  if buffer_size < null_terminated_size {
    if !size_out.is_null() {
      unsafe {
        *size_out = null_terminated_size;
      }
    }
    return luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL;
  }

  if !buffer.is_null() {
    let src = contents.as_bytes();
    let dst = unsafe { from_raw_parts_mut(buffer as *mut u8, null_terminated_size) };
    // dst 长度 = src.len() + 1，指针算术免越界检查
    unsafe {
      let dst = dst.as_mut_ptr();
      copy_nonoverlapping(src.as_ptr(), dst, src.len());
      *dst.add(src.len()) = 0;
    }
  }

  if !size_out.is_null() {
    unsafe {
      *size_out = null_terminated_size;
    }
  }

  luarequire_WriteResult::WRITE_SUCCESS
}
