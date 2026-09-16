use core::ffi::c_char;
pub type HostUserdataOperationBytecodeType = Option<
  unsafe extern "C-unwind" fn(r#type: u8, member: *const c_char, member_length: usize) -> u8,
>;
