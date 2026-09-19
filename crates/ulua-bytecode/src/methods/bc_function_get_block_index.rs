use core::ptr;

use crate::records::{bc_block::BcBlock, bc_function::BcFunction};

impl BcFunction {
  /// cpp `BcFunction::getBlockIndex`（`BytecodeGraph.h`）：由块指针反查下标。
  ///
  /// 只能用于本函数 `blocks` 里的块。cpp 靠 `LUAU_ASSERT` + 指针相减实现，release
  /// 下断言被编译掉后对非同源指针做 `offset_from` 属 UB；这里改为逐元素 `ptr::eq`
  /// 定位，找不到时 panic（可诊断，且不再有未定义行为）。签保持 `-> u32` 不变。
  pub fn get_block_index(&self, block: &BcBlock) -> u32 {
    let index = self
      .blocks
      .iter()
      .position(|candidate| ptr::eq(candidate, block))
      .unwrap_or_else(|| panic!("BcFunction::get_block_index: block is not from this function"));

    u32::try_from(index).expect("BcFunction::get_block_index: block index overflows u32")
  }
}
