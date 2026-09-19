extern crate alloc;

use crate::records::{
  base_code_gen_context::BaseCodeGenContext, shared_code_allocator::SharedCodeAllocator,
};

// Not Clone: a code-gen context owns a CodeAllocator (mmap'd executable
// memory) and the shared allocator — non-copyable in C++ too.
// Default 仅用于构造前的占位（全 null/空容器），真实初始化由
// standalone_code_gen_context_standalone_code_gen_context 以 ptr::write 整体覆盖。
#[derive(Debug, Default)]
#[repr(C)]
pub struct StandaloneCodeGenContext {
  pub base: BaseCodeGenContext,
  pub(crate) shared_allocator: SharedCodeAllocator,
}
