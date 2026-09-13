use core::ffi::c_void;
pub type AllocationCallback = unsafe extern "C-unwind" fn(
  context: *mut c_void,
  old_pointer: *mut c_void,
  old_size: usize,
  new_pointer: *mut c_void,
  new_size: usize,
);
