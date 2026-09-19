use core::ptr::null_mut;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    const_prop_in_block::const_prop_in_block, get_live_out_value_count::get_live_out_value_count,
    save_block_exit_state::save_block_exit_state,
    setup_block_entry_state_optimize_const_prop::setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a_ref::op_a_ref},
  records::{
    const_prop_state::ConstPropState,
    ir_block::{IrBlock, K_BLOCK_FLAG_SAFE_ENV_CHECK},
    ir_builder::IrBuilder,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn const_prop_in_block_chain(
  build: &mut IrBuilder,
  visited: &mut [u8],
  mut block: *mut IrBlock,
  state: &mut ConstPropState,
) {
  let function = &mut build.function as *mut _;

  state.clear();

  unsafe {
    setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state(
      build,
      &mut *function,
      &*block,
      state,
    );
  }

  let start_sortkey = unsafe { (*block).sortkey };
  let mut chain_pos = 0u32;

  while !block.is_null() {
    let block_idx = unsafe { (&*function).get_block_index(&*block) };
    CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
    visited[block_idx as usize] = 1;

    unsafe {
      if state.in_safe_env && ((*block).flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        (*block).flags &= !K_BLOCK_FLAG_SAFE_ENV_CHECK;
      }

      const_prop_in_block(build, &mut *block, state);

      if (*block).kind == IrBlockKind::Dead {
        break;
      }

      (*block).sortkey = start_sortkey;
      (*block).chainkey = chain_pos;
      chain_pos += 1;

      // cpp: `IrInst& termInst = function.instructions[block->finish];`
      // 链遍历是热路径，只取终止指令的 Copy 字段（cmd 与第 0 个操作数），
      // 不再克隆整条 IrInst（含操作数 SmallVector）。
      let (term_cmd, target_op) = {
        let term = &(&(*function).instructions)[(*block).finish as usize];
        (term.cmd, op_a_ref(term))
      };
      let mut next_block: *mut IrBlock = null_mut();

      if term_cmd == IrCmd::JUMP && target_op.kind() == IrOpKind::Block {
        let target_idx = target_op.index();
        let target = &mut (&mut (*function).blocks)[target_idx as usize] as *mut IrBlock;

        if (*target).use_count == 1
          && visited[target_idx as usize] == 0
          && (*target).kind != IrBlockKind::Fallback
        {
          // cpp OptimizeConstProp.cpp:3672-3673：live-out 非空直接终止链，出口状态不落盘
          if get_live_out_value_count(&mut *function, &mut *target) != 0 {
            break;
          }

          (*block).expected_next_block = target_idx;
          next_block = target;
        }
      }

      save_block_exit_state(&mut *function, &*block, state);

      block = next_block;
    }
  }
}
