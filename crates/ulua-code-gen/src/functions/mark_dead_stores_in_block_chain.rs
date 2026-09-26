use alloc::vec::Vec;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    kill_ir_utils::kill_ir_function_ir_inst_at,
    mark_dead_stores_in_block::mark_dead_stores_in_block,
    setup_block_entry_state_optimize_dead_store::setup_block_entry_state_ir_function_ir_block_remove_dead_store_state,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_builder::ConstantMap, ir_function::IrFunction, remove_dead_store_state::RemoveDeadStoreState,
  },
};

/// Buffer 写指令全集（两处 DSE 收尾扫描共用）
const K_BUFFER_WRITE_CMDS: [IrCmd; 6] = [
  IrCmd::BufferWritei8,
  IrCmd::BufferWritei16,
  IrCmd::BufferWritei32,
  IrCmd::BufferWritei64,
  IrCmd::BufferWritef32,
  IrCmd::BufferWritef64,
];

#[inline]
fn is_buffer_write(cmd: IrCmd) -> bool {
  K_BUFFER_WRITE_CMDS.contains(&cmd)
}

/// 分配回收首遍计数：把块内指令的 use_count 写入 uses[]，
/// buffer 写指令对分配指针操作数递减计数；返回是否有分配被写到零使用。
fn recount_uses(function: &IrFunction, uses: &mut [u32], start: u32, finish: u32) -> bool {
  let mut found_unused = false;

  for (offset, inst) in function.instructions[start as usize..=finish as usize]
    .iter()
    .enumerate()
  {
    let index = start as usize + offset;
    uses[index] = inst.use_count as u32;

    if is_buffer_write(inst.cmd) {
      let op_a_index = inst.ops[0].index() as usize;
      uses[op_a_index] -= 1;

      found_unused |= uses[op_a_index] == 0;
    }
  }

  found_unused
}

/// `state` 不再自指函数：函数视图 `function` 与 `constant_map` 由入口字段解构后
/// 独立贯穿全链，块/指令一律索引化访问（对齐 const-prop 波模式），
/// 原「构造期一次交割 `*mut IrFunction` + 访问器派生借用」的 chokepoint 整体拆除。
pub fn mark_dead_stores_in_block_chain(
  function: &mut IrFunction,
  constant_map: &mut ConstantMap,
  visited: &mut [u8],
  remaining_uses: &mut Vec<u32>,
  block_idx_chain: &mut Vec<u32>,
  all_recorded_vm_exit_syncs: &mut Vec<u32>,
  start_block_idx: u32,
) {
  let mut state =
    RemoveDeadStoreState::remove_dead_store_state_remove_dead_store_state(remaining_uses);

  // 本链会被多次访问以清理无引用的临时值
  // 清空复用的存储空间
  block_idx_chain.clear();

  setup_block_entry_state_ir_function_ir_block_remove_dead_store_state(
    function,
    start_block_idx,
    &mut state,
  );

  let mut cur: i64 = start_block_idx as i64;

  while cur >= 0 {
    let block_idx = cur as u32;

    CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
    visited[block_idx as usize] = 1;

    block_idx_chain.push(block_idx);

    // 块索引直接下传，块内借用由 mark_dead_stores_in_block 即时派生
    mark_dead_stores_in_block(function, constant_map, block_idx, &mut state);

    // 快照终止指令（JUMP 目标）字段，借用即取即释
    let (term_cmd, term_op_a) = {
      let finish = function.blocks[block_idx as usize].finish;
      let term = &function.instructions[finish as usize];
      (term.cmd, term.ops[0])
    };

    let mut next: i64 = -1;

    // 跳入单 user（即当前 block）的无条件 jump 允许继续优化
    if term_cmd == IrCmd::JUMP && term_op_a.kind() == IrOpKind::Block {
      let target_idx = term_op_a.index();
      let (target_use_count, target_kind) = {
        let target = &function.blocks[target_idx as usize];
        (target.use_count, target.kind)
      };

      if target_use_count == 1
        && visited[target_idx as usize] == 0
        && target_kind != IrBlockKind::Fallback
      {
        // 若本 block 在 lowering 顺序上未与 target 粘连，无法在 ExitSync block 捕获其残余 store
        // (cpp 无条件执行；LuauCodegenVmExitSyncFix 为 Rust 发明的假 flag，已随上游删除)
        let expected_next = function.blocks[block_idx as usize].expected_next_block;
        if expected_next != target_idx {
          state.invalidate_value_propagation(function);
        }

        next = target_idx as i64;
      }
    }

    cur = next;
  }

  state.prune_vm_exit_info(function);

  all_recorded_vm_exit_syncs.extend_from_slice(&state.recorded_vm_exit_syncs);

  // 若有分配指令，检查 DSE 后它们是否还有 'read' use
  if state.has_allocations {
    let mut found_unused = false;

    // 移除写入这些分配的指令中的 use
    for &b_idx in block_idx_chain.iter() {
      let block = &function.blocks[b_idx as usize];
      let (bstart, bfinish) = (block.start, block.finish);

      // 原实现为 while(index<=finish)：start>finish 时整块跳过，range 需显式判空
      if bstart > bfinish {
        continue;
      }

      // 函数只读再借用与计数可写借用分属两个对象，一体循环无拆分 chokepoint
      found_unused |= recount_uses(function, state.remaining_uses_mut(), bstart, bfinish);
    }

    // 若这些写指令是分配的唯一 user，将其移除
    if found_unused {
      for &b_idx in block_idx_chain.iter() {
        let block = &function.blocks[b_idx as usize];
        let (bstart, bfinish) = (block.start, block.finish);

        for index in bstart..=bfinish {
          let op_a_index = {
            let inst = &function.instructions[index as usize];

            // 仅 BufferWrite 读操作数；ops[0] 为分配指针操作数
            if is_buffer_write(inst.cmd) {
              inst.ops[0].index()
            } else {
              continue;
            }
          };

          if state.remaining_uses_ref()[op_a_index as usize] == 0
            && function.instructions[op_a_index as usize].cmd == IrCmd::NewUserdata
          {
            kill_ir_function_ir_inst_at(function, index);
          }
        }
      }
    }
  }
}
