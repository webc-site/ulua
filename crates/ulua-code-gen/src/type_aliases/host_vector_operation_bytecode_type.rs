use core::ffi::c_char;
pub type HostVectorOperationBytecodeType =
  Option<unsafe extern "C-unwind" fn(member: *const c_char, member_length: usize) -> u8>;
