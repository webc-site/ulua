extern crate alloc;

use crate::records::{
  base_code_gen_context::BaseCodeGenContext, shared_code_allocator::SharedCodeAllocator,
};

#[derive(Debug)]
#[repr(C)]
pub struct SharedCodeGenContext {
  pub base: BaseCodeGenContext,
  pub(crate) shared_allocator: SharedCodeAllocator,
}
