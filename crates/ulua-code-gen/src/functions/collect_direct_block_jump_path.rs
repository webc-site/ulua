use alloc::vec::Vec;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    get_live_in_out_value_count::get_live_in_out_value_count,
    get_live_out_value_count::get_live_out_value_count,
    try_get_next_block_in_chain::try_get_next_block_idx_in_chain,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

/// cpp `includeBlockInLinearPath`：CALL 完成后不会回到线性块，
/// 因此含 CALL 的块不能并入线性路径克隆。
/// cpp 另有 `FFlag::LuauCodegenNoLinearFastpcall && INVOKE_FASTPCALL` 检查，
/// 该指令在本仓 IrCmd 中尚未移植（上游对应 cpp OptimizeConstProp.cpp:3738）；
/// 移植 InvokeFastpcall 时须一并恢复该旗标（原声明在 round-2 作为零读取死旗标删除）。
fn include_block_in_linear_path(function: &IrFunction, block: &IrBlock) -> bool {
  function.instructions[block.start as usize..=block.finish as usize]
    .iter()
    .all(|inst| inst.cmd != IrCmd::CALL)
}

/// 起始块以索引传入，链上所有块沿索引推进，无需裸指针前置条件。
pub fn collect_direct_block_jump_path(
  function: &mut IrFunction,
  visited: &mut [u8],
  starting_idx: u32,
) -> Vec<u32> {
  CODEGEN_ASSERT!(get_live_out_value_count(function, starting_idx) == 0);

  let mut path = Vec::new();
  let mut block = Some(starting_idx);

  while let Some(block_idx) = block {
    let mut next_block: Option<u32> = None;

    let (is_jump, target_op) = {
      let finish = function.blocks[block_idx as usize].finish;
      let term_inst = &mut function.instructions[finish as usize];
      (term_inst.cmd == IrCmd::JUMP, op_a(term_inst))
    };

    if is_jump && target_op.kind() == IrOpKind::Block {
      let target_idx = target_op.index();

      if visited[target_idx as usize] == 0
        && function.blocks[target_idx as usize].kind == IrBlockKind::Internal
      {
        let (live_ins, live_outs) = get_live_in_out_value_count(function, target_idx, true);

        if live_ins == 0 && live_outs == 0 {
          let mut chain = Vec::new();

          visited[target_idx as usize] = 1;

          chain.push(target_idx);
          let mut chain_next = target_idx;

          while let Some(next_in_chain_idx) = try_get_next_block_idx_in_chain(function, chain_next)
          {
            visited[next_in_chain_idx as usize] = 1;
            chain.push(next_in_chain_idx);
            chain_next = next_in_chain_idx;
          }

          next_block = Some(chain_next);

          // 检查块链是否都可用于线性路径（cpp allValidForInclusion）
          let all_valid_for_inclusion = chain.iter().all(|&block_idx| {
            include_block_in_linear_path(function, &function.blocks[block_idx as usize])
          });

          if !all_valid_for_inclusion {
            break;
          }

          path.extend(chain);
        }
      }
    }

    block = next_block;
  }

  path
}
