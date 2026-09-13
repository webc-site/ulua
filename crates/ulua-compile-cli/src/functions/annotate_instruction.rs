use alloc::string::String;
use core::ffi::c_void;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

pub(crate) unsafe extern "C-unwind" fn annotate_instruction(
  context: *mut c_void,
  text: &mut String,
  fid: i32,
  instpos: i32,
) {
  let bcb = unsafe { &*(context as *const BytecodeBuilder) };
  bcb.annotate_instruction(text, fid as u32, instpos as u32);
}
