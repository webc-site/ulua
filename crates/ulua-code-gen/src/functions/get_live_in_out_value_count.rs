use alloc::vec::Vec;

use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{is_pseudo::is_pseudo, try_get_next_block_in_chain::try_get_next_block_in_chain},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_op::IrOp},
};

pub fn get_live_in_out_value_count(
  function: &mut IrFunction,
  start: &mut IrBlock,
  visit_chain: bool,
) -> (u32, u32) {
  let mut blocks = Vec::new();

  if visit_chain {
    let mut block = start as *mut IrBlock;
    while !block.is_null() {
      blocks.push(function.get_block_index(unsafe { &*block }));
      block = try_get_next_block_in_chain(function, unsafe { &mut *block });
    }
  } else {
    blocks.push(function.get_block_index(start));
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
          let mut found = false;
          for &b_idx in &blocks {
            let b = &function.blocks[b_idx as usize];
            if op.index() >= b.start && op.index() <= b.finish {
              CODEGEN_ASSERT!(live_outs > 0);
              live_outs -= 1;
              found = true;
              break;
            }
          }

          if !found {
            live_ins += 1;
          }
        }
      }
    }
  }

  (live_ins, live_outs)
}
