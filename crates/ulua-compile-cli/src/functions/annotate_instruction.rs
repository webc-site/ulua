use alloc::string::String;
use core::ffi::c_void;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

/// cpp `annotateInstruction`: 把 BytecodeBuilder 作为 annotator 上下文
pub(crate) unsafe extern "C-unwind" fn annotate_instruction(
  context: *mut c_void,
  text: &mut String,
  fid: i32,
  instpos: i32,
) {
  // SAFETY: annotator_context 由 compile_file 设为存活的 &mut BytecodeBuilder
  let bcb = unsafe { &*(context as *const BytecodeBuilder) };
  bcb.annotate_instruction(text, fid as u32, instpos as u32);
}
