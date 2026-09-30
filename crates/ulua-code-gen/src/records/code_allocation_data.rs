/// cpp `CodeAllocator::CodeAllocationData`：`default()`（全 null/0）即「分配失败/无分配」
/// 哨兵；字段是裸机器地址，成功值直接交给 JIT 代码当入口/基址消费（`deallocate` 判空回收）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct CodeAllocationData {
  pub start: *mut u8,
  pub size: usize,
  pub code_start: *mut u8,
  pub allocation_start: *mut u8,
  pub allocation_size: usize,
}
