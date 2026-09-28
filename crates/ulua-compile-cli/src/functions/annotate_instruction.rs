use alloc::string::String;
use core::ffi::c_void;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

/// cpp `annotateInstruction`: 把 BytecodeBuilder 作为 annotator 上下文
///
/// `// 真边界`：本函数唯一消费点是 `ulua-code-gen` 的 `AssemblyOptions::annotator`
/// 槽（`AnnotatorFn = Option<unsafe extern "C-unwind" fn(*mut c_void, …)>`，
/// lowering 在 C ABI 下调用，lower_impl 注释同此认定），故签名保留
/// `*mut c_void`（review.md §10/§7：`ffi` 类型只准存在于真 C-ABI 边界）。
/// 边界内第一步即恢复 Rust 形态（`&BytecodeBuilder`），CLI 其余内部面零裸指针。
///
/// # Safety
/// `context` 必须是 `compile_file` 用 `addr_of_mut!(bcb)` 一次性取到的
/// `BytecodeBuilder` 地址（本 CLI 注册该回调时唯一来源），且在整次 codegen
/// 期间存活、回调时无其它可变借用。
pub(crate) unsafe extern "C-unwind" fn annotate_instruction(
  context: *mut c_void,
  text: &mut String,
  fid: i32,
  instpos: i32,
) {
  // Safety: context 是 compile_file 用 addr_of_mut!(bcb) 一次性取到的
  // BytecodeBuilder 地址：bcb 在整次 codegen 期间存活（同一栈帧），回调为单线程
  // 同步调用且回调期内调用方不持有该 place 的 &mut，故无并发写。
  let bcb = unsafe { &*(context as *const BytecodeBuilder) };
  bcb.annotate_instruction(text, fid as u32, instpos as u32);
}
