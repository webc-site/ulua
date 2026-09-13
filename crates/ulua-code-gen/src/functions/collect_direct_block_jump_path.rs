use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    get_live_in_out_value_count::get_live_in_out_value_count,
    get_live_out_value_count::get_live_out_value_count,
    try_get_next_block_in_chain::try_get_next_block_in_chain,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn collect_direct_block_jump_path(
  function: &mut IrFunction,
  visited: &mut [u8],
  mut block: *mut IrBlock,
) -> Vec<u32> {
  CODEGEN_ASSERT!(get_live_out_value_count(function, unsafe { &mut *block }) == 0);

  let mut path = Vec::new();

  while !block.is_null() {
    let mut next_block: *mut IrBlock = null_mut();

    let (is_jump, target_op) = {
      let term_inst = &mut function.instructions[unsafe { (*block).finish } as usize];
      (term_inst.cmd == IrCmd::JUMP, op_a(term_inst))
    };

    if is_jump && target_op.kind() == IrOpKind::Block {
      let target_block = function.block_op(target_op) as *mut IrBlock;
      let target_idx = function.get_block_index(unsafe { &*target_block });

      if visited[target_idx as usize] == 0
        && unsafe { (*target_block).kind } == IrBlockKind::Internal
      {
        let (live_ins, live_outs) =
          get_live_in_out_value_count(function, unsafe { &mut *target_block }, true);

        if live_ins == 0 && live_outs == 0 {
          visited[target_idx as usize] = 1;
          path.push(target_idx);

          next_block = target_block;

          loop {
            let next_in_chain = try_get_next_block_in_chain(function, unsafe { &mut *next_block });
            if !next_in_chain.is_null() {
              let next_in_chain_idx = function.get_block_index(unsafe { &*next_in_chain });
              visited[next_in_chain_idx as usize] = 1;
              path.push(next_in_chain_idx);
              next_block = next_in_chain;
            } else {
              break;
            }
          }
        }
      }
    }

    block = next_block;
  }

  path
}
