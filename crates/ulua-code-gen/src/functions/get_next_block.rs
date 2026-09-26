use crate::{enums::ir_block_kind::IrBlockKind, records::ir_function::IrFunction};

/// 返回排序表中第 i 块之后的首个非死块索引（None 表示不存在）。
/// 原形态返回 `&mut IrBlock`（含哨兵 dummy 块），调用方仅用于与
/// `expected_next_block` 的一致性断言，索引形态即其等价物。
pub fn get_next_block(function: &IrFunction, sorted_blocks: &[u32], i: usize) -> Option<u32> {
  sorted_blocks
    .iter()
    .skip(i + 1)
    .map(|&idx| idx as usize)
    .find(|&idx| function.blocks[idx].kind != IrBlockKind::Dead)
    .map(|idx| idx as u32)
}
