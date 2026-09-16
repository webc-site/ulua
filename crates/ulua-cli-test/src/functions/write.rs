use core::{ffi::c_char, ptr::copy_nonoverlapping};

use ulua_require::enums::luarequire_write_result::luarequire_WriteResult;

pub(crate) unsafe fn write(
  contents: Option<&str>,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  let contents = match contents {
    Some(c) => c,
    None => return luarequire_WriteResult::WRITE_FAILURE,
  };

  let null_terminated_size = contents.len() + 1;

  if buffer_size < null_terminated_size {
    unsafe {
      *size_out = null_terminated_size;
    }
    return luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL;
  }

  unsafe {
    *size_out = null_terminated_size;
    copy_nonoverlapping(contents.as_ptr(), buffer as *mut u8, contents.len());
    *buffer.add(contents.len()) = 0;
  }

  luarequire_WriteResult::WRITE_SUCCESS
}
