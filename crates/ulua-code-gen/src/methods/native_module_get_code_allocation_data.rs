use ulua_common::fflag;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{code_allocation_data::CodeAllocationData, native_module::NativeModule},
};

impl NativeModule {
  pub fn native_module_get_code_allocation_data(&self) -> CodeAllocationData {
    CODEGEN_ASSERT!(fflag::LuauCodegenFreeBlocks.get());
    self.code_allocation_data
  }
}
