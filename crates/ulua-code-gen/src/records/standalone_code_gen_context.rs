extern crate alloc;

use crate::records::{
  base_code_gen_context::BaseCodeGenContext, shared_code_allocator::SharedCodeAllocator,
};

// Not Clone: a code-gen context owns a CodeAllocator (mmap'd executable
// memory) and the shared allocator — non-copyable in C++ too.
#[derive(Debug)]
#[repr(C)]
pub struct StandaloneCodeGenContext {
  pub base: BaseCodeGenContext,
  pub(crate) shared_allocator: SharedCodeAllocator,
}
