use ulua_common::FFlag;

use crate::{
  functions::make_pages_not_executable_code_allocator::make_pages_not_executable,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{code_allocation_data::CodeAllocationData, code_allocator::CodeAllocator},
};

impl CodeAllocator {
  pub fn deallocate(&mut self, code_allocation_data: CodeAllocationData) {
    CODEGEN_ASSERT!(FFlag::LuauCodegenFreeBlocks.get());

    if code_allocation_data.allocation_start.is_null() {
      return;
    }

    let result = make_pages_not_executable(
      code_allocation_data.allocation_start,
      code_allocation_data.allocation_size,
    );
    CODEGEN_ASSERT!(result);

    CODEGEN_ASSERT!(self.live_allocations != 0);
    self.live_allocations -= 1;
  }
}
