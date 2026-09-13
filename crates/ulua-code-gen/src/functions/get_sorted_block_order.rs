use alloc::vec::Vec;

use ulua_common::FFlag;

use crate::{
  enums::ir_block_kind::IrBlockKind, functions::get_block_kind_priority::get_block_kind_priority,
  records::ir_function::IrFunction,
};

pub fn get_sorted_block_order(function: &mut IrFunction) -> Vec<u32> {
  let mut sorted_blocks: Vec<u32> = (0..function.blocks.len() as u32).collect();

  // Native-only helper for sorting blocks. This matches the C++ comparator semantics:
  // - If LuauCodegenVmExitSync is enabled: use get_block_kind_priority ordering first,
  //   and fall back to sortkey/chainkey afterwards.
  // - Otherwise: ensure Fallback blocks are placed at the end, then order by sortkey/chainkey.
  let vm_exit_sync = FFlag::LuauCodegenVmExitSync.get();

  sorted_blocks.sort_by(|&idx_a, &idx_b| {
    let a = &function.blocks[idx_a as usize];
    let b = &function.blocks[idx_b as usize];

    if vm_exit_sync {
      let (pri_a, pri_b) = (
        get_block_kind_priority(a.kind),
        get_block_kind_priority(b.kind),
      );
      if pri_a != pri_b {
        return pri_a.cmp(&pri_b);
      }
    } else {
      let (a_is_fallback, b_is_fallback) = (
        a.kind == IrBlockKind::Fallback,
        b.kind == IrBlockKind::Fallback,
      );
      if a_is_fallback != b_is_fallback {
        return a_is_fallback.cmp(&b_is_fallback);
      }
    }

    if a.sortkey != b.sortkey {
      return a.sortkey.cmp(&b.sortkey);
    }

    a.chainkey.cmp(&b.chainkey)
  });

  sorted_blocks
}
