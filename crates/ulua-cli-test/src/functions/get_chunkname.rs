use core::{
  ffi::{c_char, c_void},
  ptr::copy_nonoverlapping,
};

use ulua_require::enums::luarequire_write_result::luarequire_WriteResult;

use crate::records::repl_requirer::ReplRequirer;

pub unsafe extern "C-unwind" fn get_chunkname(
  _l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  let req = ctx as *mut ReplRequirer;
  let path = unsafe { (*req).vfs.get_file_path() };
  let null_terminated_size = path.len() + 2;

  if buffer_size < null_terminated_size {
    unsafe {
      *size_out = null_terminated_size;
    }
    return luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL;
  }

  unsafe {
    *size_out = null_terminated_size;
    *(buffer as *mut u8) = b'@';
    copy_nonoverlapping(path.as_ptr(), (buffer as *mut u8).add(1), path.len());
    *buffer.add(path.len() + 1) = 0;
  }

  luarequire_WriteResult::WRITE_SUCCESS
}
