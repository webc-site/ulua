use alloc::vec::Vec;

use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{is_pseudo::is_pseudo, try_get_next_block_in_chain::try_get_next_block_idx_in_chain},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

/// 起始块以索引传入；链式遍历全程只持有索引，消除块裸指针往返。
pub fn get_live_in_out_value_count(
  function: &mut IrFunction,
  start_idx: u32,
  visit_chain: bool,
) -> (u32, u32) {
  let mut blocks = Vec::new();

  if visit_chain {
    let mut block = Some(start_idx);
    while let Some(idx) = block {
      blocks.push(idx);
      block = try_get_next_block_idx_in_chain(function, idx);
    }
  } else {
    blocks.push(start_idx);
  }

  let mut live_ins = 0;
  let mut live_outs = 0;

  for &block_idx in &blocks {
    let block = &function.blocks[block_idx as usize];

    for inst_idx in block.start..=block.finish {
      let inst = &function.instructions[inst_idx as usize];

      if is_pseudo(inst.cmd) {
        continue;
      }

      live_outs += inst.use_count as u32;

      for op in inst.ops.iter() {
        let op: IrOp = *op;
        if op.kind() == IrOpKind::Inst {
          // any() 短路即等价原 found+break 标志循环
          let defined_in_blocks = blocks.iter().any(|&b_idx| {
            let b = &function.blocks[b_idx as usize];
            if op.index() >= b.start && op.index() <= b.finish {
              CODEGEN_ASSERT!(live_outs > 0);
              live_outs -= 1;
              true
            } else {
              false
            }
          });

          if !defined_in_blocks {
            live_ins += 1;
          }
        }
      }
    }
  }

  (live_ins, live_outs)
}
