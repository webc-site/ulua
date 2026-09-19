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
  // SAFETY: context 是 compile_file 用 addr_of_mut!(bcb) 一次性取到的
  // BytecodeBuilder 地址：bcb 在整次 codegen 期间存活（同一栈帧），回调为单线程
  // 同步调用且回调期内调用方不持有该 place 的 &mut，故无并发写。
  let bcb = unsafe { &*(context as *const BytecodeBuilder) };
  bcb.annotate_instruction(text, fid as u32, instpos as u32);
}
