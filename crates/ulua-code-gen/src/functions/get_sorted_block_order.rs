use alloc::vec::Vec;

use crate::{
  functions::get_block_kind_priority::get_block_kind_priority, records::ir_function::IrFunction,
};

pub fn get_sorted_block_order(function: &mut IrFunction) -> Vec<u32> {
  let mut sorted_blocks: Vec<u32> = (0..function.blocks.len() as u32).collect();

  // Fallback 块放末尾，其后是 exit sync 块（cpp IrUtils.cpp:1806）
  sorted_blocks.sort_by(|&idx_a, &idx_b| {
    let a = &function.blocks[idx_a as usize];
    let b = &function.blocks[idx_b as usize];

    let (pri_a, pri_b) = (
      get_block_kind_priority(a.kind),
      get_block_kind_priority(b.kind),
    );
    if pri_a != pri_b {
      return pri_a.cmp(&pri_b);
    }

    if a.sortkey != b.sortkey {
      return a.sortkey.cmp(&b.sortkey);
    }

    a.chainkey.cmp(&b.chainkey)
  });

  sorted_blocks
}
